use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    ast::{
        base::Node,
        expression::{Expression, InfixOperatorType},
        statements::{Program, Statement},
    },
    code::{
        OpCode,
        definitions::{Byte, Instructions, OpCodes},
        make::make,
        symbol_table::SymbolTable,
    },
    object::{CompiledFunctionEntry, Object},
    tokens::{self, Token, TokenKind},
};

use super::symbol_table::SymbolType;

#[derive(Debug)]
pub enum CompilationError {
    UnexpectedSymbol(Rc<Token>),
    NotImplementedYet(Rc<Expression>),
    UnknownOperator(Rc<Token>, InfixOperatorType),
    UndefinedVariable(Rc<Token>, String),
    ParameterOfFunctionHasToBeIdentifier(Rc<Token>),
    WrongNumberOfArguments {
        token: Rc<Token>,
        expected: usize,
        provided: usize,
    },
    BuiltinCannotBeSet(String),
    FreeCannotBeSet(String),
    FunctionCannotBeSet(String),
}

pub fn compile<T: Node>(node: T) -> Result<Bytecode, Vec<CompilationError>> {
    let mut container = Worker::new();
    container.compile(&node);
    container.into()
}

#[derive(Clone)]
pub(crate) struct EmitedInstruction {
    pub(crate) opcode: OpCodes,
    position: usize,
}

#[derive(Clone)]
pub(crate) struct CompilationScope {
    pub(crate) instructions: Vec<Byte>,
    pub(crate) last_instruction: Option<EmitedInstruction>,
    pub(crate) previous_instruction: Option<EmitedInstruction>, //can temporary show not correct values, it is
}

impl CompilationScope {
    fn new() -> Self {
        CompilationScope {
            instructions: vec![],
            last_instruction: None,
            previous_instruction: None,
        }
    }
}

pub(crate) struct Worker {
    pub(crate) constants: Vec<Object>,
    pub(crate) errors: Vec<CompilationError>,
    pub(crate) scopes: Vec<CompilationScope>,
    pub(crate) scope_index: usize,
    pub(crate) symbol_table: Rc<RefCell<SymbolTable>>,
    //TODO: provide symbol map: bytes -> position, stil naive no stack trace
}

macro_rules! scope {
    ($self:expr) => {
        $self
            .scopes
            .get($self.scope_index)
            .take()
            .expect("Scope has to be defined")
    };
}

macro_rules! scope_mut {
    ($self:expr) => {
        &mut $self
            .scopes
            .get_mut($self.scope_index)
            .take()
            .expect("Scope has to be defined")
    };
}
impl Worker {
    pub(crate) fn new() -> Self {
        let main_scope = CompilationScope::new();
        Worker {
            constants: vec![],
            errors: vec![],
            symbol_table: SymbolTable::new_table(),
            scopes: vec![main_scope],
            scope_index: 0,
        }
    }

    fn add_errors(&mut self, error: CompilationError) {
        self.errors.push(error);
    }
    fn add_constant(&mut self, object: Object) -> u16 {
        self.constants.push(object);
        return (self.constants.len() - 1) as u16;
    }

    pub(crate) fn emit_op_code(&mut self, op_codes: OpCodes) {
        self.emit(op_codes.into(), &[]);
    }

    fn emit(&mut self, op_codes: OpCodes, operands: &[u16]) -> usize {
        let instructions = make(op_codes.into(), operands);
        let possition = self.add_instruction(instructions);
        self.set_last_emited(op_codes, possition);
        return possition;
    }

    fn current_instructions_lenght(&self) -> usize {
        scope!(self).instructions.len()
    }

    fn change_bytecode(&mut self, index: usize, byte: Byte) {
        let current_instructions = &mut scope_mut!(self).instructions;
        current_instructions[index] = byte;
    }
    fn read_bytecode(&mut self, index: usize) -> Byte {
        let current_instructions = &mut scope_mut!(self).instructions;
        current_instructions[index].clone()
    }

    fn add_instruction(&mut self, instructions: Instructions) -> usize {
        let current_instructions = &mut scope_mut!(self).instructions;
        let previous_position = current_instructions.len();
        for byte in instructions.bytes() {
            current_instructions.push(byte.clone());
        }
        previous_position
    }

    pub(crate) fn enter_scope(&mut self) {
        let scope = CompilationScope::new();
        self.symbol_table = SymbolTable::enclosed(&self.symbol_table);
        self.scopes.push(scope);
        self.scope_index += 1;
    }

    pub(crate) fn leave_scope(&mut self) -> Instructions {
        let scope = self.scopes.pop().take().expect("Scope was not defined");
        self.scope_index -= 1;
        self.symbol_table = SymbolTable::outer(&self.symbol_table);
        Instructions(scope.instructions)
    }

    fn compile<T: Node>(&mut self, node: &T) {
        let statement = node.as_any().downcast_ref::<Statement>();
        if let Some(statement) = statement {
            self.compile_statement(statement);
            return;
        }
        let program = node.as_any().downcast_ref::<Program>();
        if let Some(program) = program {
            self.compile_program(program);
            return;
        }

        let expression = node.as_any().downcast_ref::<Expression>();
        if let Some(expression) = expression {
            self.compile_expression(expression);
            return;
        }
        panic!("Should never reach here, node: {:?}", node);
    }

    fn compile_program(&mut self, program: &Program) {
        for statement in program.statements.clone() {
            self.compile_statement(&statement);
        }
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let { token, name, value } => {
                let name = match name {
                    Expression::Identifier(token) => match &token.kind {
                        crate::tokens::TokenKind::Identifier(v) => v,
                        _ => {
                            self.add_errors(CompilationError::UnexpectedSymbol(token.clone()));
                            return;
                        }
                    },
                    _ => {
                        self.add_errors(CompilationError::UnexpectedSymbol(token.clone()));
                        return;
                    }
                };
                self.compile_let(name.to_string(), value)
            }
            Statement::Return {
                token: _,
                return_value,
            } => {
                self.compile_expression(return_value);
                self.emit_op_code(OpCodes::ReturnValue);
            }
            Statement::AExpression {
                token: _,
                expression,
            } => {
                self.compile_expression(expression);
                self.emit_op_code(OpCodes::Pop);
            }
            Statement::Block {
                token: _,
                statements,
            } => {
                statements
                    .iter()
                    .for_each(|s| self.compile_statement(&s.clone()));
            }
        }
    }

    fn compile_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::IntegerLiteral(token) => {
                let value = match token.kind {
                    crate::tokens::TokenKind::Integer(v) => v,
                    _ => {
                        self.add_errors(CompilationError::UnexpectedSymbol(token.clone()));
                        return;
                    }
                };
                let value = Object::Int(value.into());
                let constant_possition = self.add_constant(value);
                self.emit(OpCodes::Constant, &[constant_possition]);
            }
            Expression::StringLiteral(token) => {
                let value = match &token.kind {
                    crate::tokens::TokenKind::StringLiteral(v) => v,
                    _ => {
                        self.add_errors(CompilationError::UnexpectedSymbol(token.clone()));
                        return;
                    }
                };
                let value = Object::String(value.into());
                let constant_possition = self.add_constant(value);
                self.emit(OpCodes::Constant, &[constant_possition]);
            }
            Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                match operator {
                    InfixOperatorType::LessThan => {
                        self.compile_expression(&right);
                        self.compile_expression(&left);
                    }
                    _ => {
                        self.compile_expression(&left);
                        self.compile_expression(&right);
                    }
                }
                match operator {
                    InfixOperatorType::Plus => self.emit_op_code(OpCodes::Add),
                    InfixOperatorType::Divide => self.emit_op_code(OpCodes::Divide),
                    InfixOperatorType::Minus => self.emit_op_code(OpCodes::Subtitute),
                    InfixOperatorType::Multiply => self.emit_op_code(OpCodes::Multiply),
                    InfixOperatorType::NotEqual => self.emit_op_code(OpCodes::NotEqual),
                    InfixOperatorType::LessThan => self.emit_op_code(OpCodes::GreaterThan),
                    InfixOperatorType::GreaterThan => self.emit_op_code(OpCodes::GreaterThan),
                    InfixOperatorType::Equal => self.emit_op_code(OpCodes::Equal),
                }
            }
            Expression::BooleanLiteral { token: _, value } => {
                match *value {
                    true => self.emit(OpCodes::True, &[]),
                    false => self.emit(OpCodes::False, &[]),
                };
            }
            Expression::PrefixOperator {
                token: _,
                operator,
                right,
            } => {
                self.compile_expression(&right);
                match operator {
                    crate::ast::expression::PrefixOperatorType::Bang => {
                        self.emit_op_code(OpCodes::Bang)
                    }
                    crate::ast::expression::PrefixOperatorType::Minus => {
                        self.emit_op_code(OpCodes::Minus)
                    }
                }
            }
            Expression::AIf {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expression(&condition);
                let jump_after_consequences = self.emit(OpCodes::JumpNotTruthy, &[9999]);
                self.compile_statement(consequence.as_ref());
                if self.last_instruction_is(OpCodes::Pop) {
                    self.remove_last_pop();
                }
                let ajump_to_end_of_conditional = self.emit(OpCodes::Jump, &[9999]);
                self.change_operand(
                    jump_after_consequences,
                    &[self.current_instructions_lenght() as u16],
                );
                match alternative {
                    Some(body) => {
                        self.compile_statement(&body);
                        if self.last_instruction_is(OpCodes::Pop) {
                            self.remove_last_pop();
                        }
                    }
                    None => {
                        self.emit_op_code(OpCodes::Null);
                    }
                }
                self.change_operand(
                    ajump_to_end_of_conditional,
                    &[self.current_instructions_lenght() as u16],
                );
            }
            Expression::Identifier(token) => {
                match &token.kind {
                    tokens::TokenKind::Identifier(v) => self.load_symbol(v, token.clone()),
                    _ => self.add_errors(CompilationError::UnexpectedSymbol(token.clone())),
                };
            }
            Expression::ArrayLiteral { token: _, elements } => {
                for element in elements {
                    self.compile(element);
                }
                //TODO exception when usize larger than u16
                self.emit(OpCodes::Array, &[elements.len() as u16]);
            }
            Expression::MapLiteral { token: _, elements } => {
                for (key, value) in elements {
                    self.compile(key);
                    self.compile(value);
                }
                let size = elements.len() * 2;
                self.emit(OpCodes::Hash, &[size as u16]);
            }
            Expression::Index {
                token: _,
                array,
                index,
            } => {
                self.compile(array.as_ref());
                self.compile(index.as_ref());
                self.emit_op_code(OpCodes::Index);
            }
            Expression::FunctionLiteral {
                token,
                parameters,
                body,
                name,
            } => {
                self.enter_scope();
                if let Some(found_name) = name {
                    SymbolTable::define_function_name(&self.symbol_table, found_name);
                }
                for parameter in parameters.clone().as_ref() {
                    match parameter {
                        Expression::Identifier(id_token) => match &id_token.kind {
                            TokenKind::Identifier(value) => {
                                SymbolTable::define(&self.symbol_table, &value);
                            }
                            _ => self.add_errors(
                                CompilationError::ParameterOfFunctionHasToBeIdentifier(
                                    id_token.clone(),
                                ),
                            ),
                        },
                        _ => self.add_errors(
                            CompilationError::ParameterOfFunctionHasToBeIdentifier(token.clone()),
                        ),
                    }
                }

                self.compile(body.as_ref());
                if self.last_instruction_is(OpCodes::Pop) {
                    self.replace_last_pop_with_return()
                }
                if !self.last_instruction_is(OpCodes::ReturnValue) {
                    self.emit_op_code(OpCodes::ReturnNone);
                }
                let number_of_locals = SymbolTable::number_of_locals(&self.symbol_table);
                let free_symbols = &self.symbol_table.borrow().free_symbols.clone(); //Has to
                //happen before leave scope, otherwise free variables will be lost after leaving
                //scope
                let instructions = self.leave_scope();
                for free in free_symbols {
                    self.load_symbol(&free.name, token.clone());
                }
                let compiled_function = Object::CompiledFunction(CompiledFunctionEntry {
                    instructions,
                    number_of_locals,
                    number_of_parameters: parameters.len(),
                });
                let constant_position = self.add_constant(compiled_function);
                self.emit(
                    OpCodes::Closure,
                    &[constant_position, free_symbols.len() as u16],
                );
            }
            Expression::Call {
                token,
                function,
                arguments,
            } => {
                self.check_call(&function, token.clone(), &arguments);
                self.compile_expression(&function);
                for argument in arguments {
                    self.compile_expression(argument);
                }
                self.emit(OpCodes::Call, &[arguments.len() as u16]);
            }
            _ => self.add_errors(CompilationError::NotImplementedYet(Rc::new(
                expression.clone(),
            ))),
        }
    }

    fn load_symbol(&mut self, name: &String, token: Rc<Token>) {
        match SymbolTable::resolve(&self.symbol_table, name) {
            Some(symbol) => {
                let code = match symbol.what_type() {
                    SymbolType::GLOBAL => OpCodes::GetGlobal,
                    SymbolType::LOCAL => OpCodes::GetLocal,
                    SymbolType::BUILTIN => OpCodes::GetBuiltin,
                    SymbolType::FREE => OpCodes::GetFree,
                    SymbolType::FUNCTION => {
                        self.emit_op_code(OpCodes::CurrentClosure);
                        return;
                    }
                };
                self.emit(code, &[symbol.index]);
            }
            None => {
                self.add_errors(CompilationError::UndefinedVariable(
                    token.clone(),
                    name.to_string(),
                ));
            }
        }
    }

    fn replace_instructions(&mut self, possition: usize, new_instruction: Instructions) {
        let mut i = 0;
        let bytes = new_instruction.bytes();
        while i < new_instruction.length() {
            self.change_bytecode(possition + i, bytes[i].clone());
            i += 1;
        }
    }

    /**
     * assumption:
     * - same instruction type
     * - same operands length
     */
    fn change_operand(&mut self, operand_possition: usize, operands: &[u16]) {
        let op_code = OpCode(self.read_bytecode(operand_possition));
        let new_instruction = make(op_code, operands);
        self.replace_instructions(operand_possition, new_instruction);
    }

    fn last_instruction_is(&self, op_codes: OpCodes) -> bool {
        let scope = scope!(self);

        match scope.last_instruction {
            Some(ref v) => v.opcode == op_codes,
            None => false,
        }
    }
    fn remove_last_pop(&mut self) {
        let scope = scope_mut!(self);
        let pop_position = match scope.last_instruction {
            Some(ref v) => v.position,
            None => return,
        };
        scope.instructions.remove(pop_position);
        scope.last_instruction = scope.previous_instruction.clone();
    }

    fn set_last_emited(&mut self, op_codes: OpCodes, possition: usize) {
        let scope = scope_mut!(self);
        scope.previous_instruction = scope.last_instruction.clone();
        scope.last_instruction = Some(EmitedInstruction {
            opcode: op_codes,
            position: possition,
        });
    }

    fn compile_let(&mut self, name: String, value: &Expression) {
        let symbol = SymbolTable::define(&self.symbol_table, &name);
        self.compile_expression(value);
        let op_code = match symbol.what_type() {
            SymbolType::GLOBAL => OpCodes::SetGlobal,
            SymbolType::LOCAL => OpCodes::SetLocal,
            SymbolType::BUILTIN => {
                self.add_errors(CompilationError::BuiltinCannotBeSet(name));
                return;
            }

            SymbolType::FREE => {
                self.add_errors(CompilationError::FunctionCannotBeSet(name));
                return;
            }
            SymbolType::FUNCTION => todo!(),
        };
        self.emit(op_code, &[symbol.index]);
    }

    fn replace_last_pop_with_return(&mut self) {
        let scope = scope!(self);
        let pop_position = match scope.last_instruction {
            Some(ref v) => v.position,
            None => return,
        };
        let return_code = make(OpCodes::ReturnValue.into(), &[]);
        self.replace_instructions(pop_position, return_code);
        let scope = scope_mut!(self);
        scope
            .last_instruction
            .take()
            .expect("has to be there")
            .opcode = OpCodes::ReturnValue;
    }

    fn check_call(&mut self, function: &Expression, token: Rc<Token>, arguments: &[Expression]) {
        match function {
            Expression::FunctionLiteral {
                token: _,
                parameters,
                body: _,
                name: _,
            } => {
                if parameters.len() != arguments.len() {
                    self.add_errors(CompilationError::WrongNumberOfArguments {
                        token: token.clone(),
                        expected: parameters.len(),
                        provided: arguments.len(),
                    });
                }
            }
            _ => {}
        }
    }
}

impl From<Worker> for Result<Bytecode, Vec<CompilationError>> {
    fn from(value: Worker) -> Self {
        let errors = value.errors;
        if errors.len() > 0 {
            return Result::Err(errors);
        }
        let current_scope = scope!(value);
        Result::Ok(Bytecode {
            instructions: Instructions(current_scope.instructions.clone()),
            constants: value.constants,
        })
    }
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
