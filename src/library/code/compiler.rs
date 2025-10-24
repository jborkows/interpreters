use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    ast::{
        base::Node,
        expression::Expression,
        statements::{Program, Statement},
    },
    code::{
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
}

pub fn compile<T: Node>(node: T) -> Result<Bytecode, Vec<CompilationError>> {
    let mut container = Worker::new();
    container.compile(&node);
    container.into()
}

struct Worker {
    instructions: RefCell<Vec<Byte>>,
    constants: RefCell<Vec<Object>>,
    errors: RefCell<Vec<CompilationError>>,
}

impl Worker {
    fn new() -> Self {
        Worker {
            instructions: RefCell::new(vec![]),
            constants: RefCell::new(vec![]),
            errors: RefCell::new(vec![]),
        }
    }

    fn add_errors(&mut self, error: CompilationError) {
        self.errors.borrow_mut().push(error);
    }
    fn add_constant(&mut self, object: Object) -> u16 {
        self.constants.borrow_mut().push(object);
        return (self.constants.borrow().len() - 1) as u16;
    }
    fn emit(&mut self, op_codes: OpCodes, operands: &[u16]) -> usize {
        let instructions = make(op_codes.into(), operands);
        return self.add_instruction(instructions);
    }
    fn add_instruction(&mut self, instructions: Instructions) -> usize {
        let previous_position = self.instructions.borrow().len();
        for byte in instructions.bytes() {
            self.instructions.borrow_mut().push(byte.clone());
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
            } => self.compile_expression(expression),
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
                token,
                left,
                operator,
                right,
            } => {
                self.compile_expression(&left);
                self.compile_expression(&right);
            }
            _ => self.add_errors(CompilationError::NotImplementedYet(Rc::new(
                expression.clone(),
            ))),
        }
    }
}

impl From<Worker> for Result<Bytecode, Vec<CompilationError>> {
    fn from(value: Worker) -> Self {
        let errors = value.errors.take();
        if errors.len() > 0 {
            return Result::Err(errors);
        }
        Result::Ok(Bytecode {
            instructions: Instructions(value.instructions.take()),
            constants: value.constants.take(),
        })
    }
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
