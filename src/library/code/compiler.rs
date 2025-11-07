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
    },
    object::Object,
    tokens::Token,
};

#[derive(Debug)]
pub enum CompilationError {
    UnexpectedSymbol(Rc<Token>),
    NotImplementedYet(Rc<Expression>),
    UnknownOperator(Rc<Token>, InfixOperatorType),
}

pub fn compile<T: Node>(node: T) -> Result<Bytecode, Vec<CompilationError>> {
    let mut container = Worker::new();
    container.compile(&node);
    container.into()
}

#[derive(Clone)]
struct EmitedInstruction {
    opcode: OpCodes,
    position: usize,
}

struct Worker {
    instructions: Vec<Byte>,
    constants: Vec<Object>,
    errors: Vec<CompilationError>,
    last_instruction: Option<EmitedInstruction>,
    previous_instruction: Option<EmitedInstruction>, //can temporary show not correct values, it is
                                                     //to eliminate to pops in if statement
}

impl Worker {
    fn new() -> Self {
        Worker {
            instructions: vec![],
            constants: vec![],
            errors: vec![],
            last_instruction: None,
            previous_instruction: None,
        }
    }

    fn add_errors(&mut self, error: CompilationError) {
        self.errors.push(error);
    }
    fn add_constant(&mut self, object: Object) -> u16 {
        self.constants.push(object);
        return (self.constants.len() - 1) as u16;
    }

    fn emit_op_code(&mut self, op_codes: OpCodes) {
        self.emit(op_codes.into(), &[]);
    }

    fn emit(&mut self, op_codes: OpCodes, operands: &[u16]) -> usize {
        let instructions = make(op_codes.into(), operands);
        let possition = self.add_instruction(instructions);
        self.set_last_emited(op_codes, possition);
        return possition;
    }
    fn add_instruction(&mut self, instructions: Instructions) -> usize {
        let previous_position = self.instructions.len();
        for byte in instructions.bytes() {
            self.instructions.push(byte.clone());
        }
        previous_position
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
            Statement::Let { token, name, value } => todo!(),
            Statement::Return {
                token,
                return_value,
            } => todo!(),
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
                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }
                let ajump_to_end_of_conditional = self.emit(OpCodes::Jump, &[9999]);
                self.change_operand(jump_after_consequences, &[self.instructions.len() as u16]);
                match alternative {
                    Some(body) => {
                        self.compile_statement(&body);
                        if self.last_instruction_is_pop() {
                            self.remove_last_pop();
                        }
                    }
                    None => {
                        self.emit_op_code(OpCodes::Null);
                    }
                }
                self.change_operand(
                    ajump_to_end_of_conditional,
                    &[self.instructions.len() as u16],
                );
            }
            _ => self.add_errors(CompilationError::NotImplementedYet(Rc::new(
                expression.clone(),
            ))),
        }
    }

    fn replace_instructions(&mut self, possition: usize, new_instruction: Instructions) {
        let mut i = 0;
        let bytes = new_instruction.bytes();
        while i < new_instruction.length() {
            self.instructions[possition + i] = bytes[i].clone();
            i += 1;
        }
    }

    /**
     * assumption:
     * - same instruction type
     * - same operands length
     */
    fn change_operand(&mut self, operand_possition: usize, operands: &[u16]) {
        let op_code = OpCode(self.instructions[operand_possition].clone());
        let new_instruction = make(op_code, operands);
        self.replace_instructions(operand_possition, new_instruction);
    }

    fn last_instruction_is_pop(&self) -> bool {
        match self.last_instruction {
            Some(ref v) => v.opcode == OpCodes::Pop,
            None => false,
        }
    }
    fn remove_last_pop(&mut self) {
        let pop_position = match self.last_instruction {
            Some(ref v) => v.position,
            None => return,
        };
        self.instructions.remove(pop_position);
        self.last_instruction = self.previous_instruction.clone();
    }

    fn set_last_emited(&mut self, op_codes: OpCodes, possition: usize) {
        self.previous_instruction = self.last_instruction.clone();
        self.last_instruction = Some(EmitedInstruction {
            opcode: op_codes,
            position: possition,
        });
    }
}

impl From<Worker> for Result<Bytecode, Vec<CompilationError>> {
    fn from(value: Worker) -> Self {
        let errors = value.errors;
        if errors.len() > 0 {
            return Result::Err(errors);
        }
        Result::Ok(Bytecode {
            instructions: Instructions(value.instructions),
            constants: value.constants,
        })
    }
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
