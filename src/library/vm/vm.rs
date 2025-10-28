use std::{panic, rc::Rc};

use crate::{
    ast::expression::InfixOperatorType,
    code::{Bytecode, Instructions, OpCode, OpCodes, read_u_16},
    object::Object,
};

const STACK_SIZE: usize = 2048;
pub struct VM {
    constants: Vec<Rc<Object>>,
    instructions: Instructions,
    stack: [Rc<Object>; STACK_SIZE],
    stack_pointer: usize, //points to the next value. Top of stack is stack[stack_pointer-1]
}

impl VM {
    pub fn new(byte_code: Bytecode) -> Self {
        let constants: Vec<Rc<Object>> = byte_code
            .constants
            .iter()
            .map(|o| o.clone())
            .map(Rc::new)
            .collect::<Vec<_>>();
        VM {
            constants: constants,
            instructions: byte_code.instructions,
            stack: std::array::from_fn(|_| Rc::new(NIL)),
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        let bytes = self.instructions.bytes();
        let mut instruction_pointer = 0;
        while instruction_pointer < bytes.len() {
            let instruction: u8 = bytes.get(instruction_pointer).unwrap().into();

            match instruction {
                CONSTANT => {
                    let constant_index = read_u_16(&bytes[instruction_pointer + 1..]);
                    instruction_pointer += 2;
                    let cloned = self.constants.clone();
                    let constant = cloned.get(constant_index as usize).expect(
                        format!("Can not find constant at index {constant_index}").as_str(),
                    );
                    self.push(constant.clone());
                }
                ADD => {
                    self.binary_operation(InfixOperatorType::Plus);
                }
                SUBSTITUTE => {
                    self.binary_operation(InfixOperatorType::Minus);
                }
                MULTIPLY => {
                    self.binary_operation(InfixOperatorType::Multiply);
                }
                DIVIDE => {
                    self.binary_operation(InfixOperatorType::Divide);
                }
                POP => {
                    self.pop();
                }
                _ => panic!("Don't know what to do with {instruction}"),
            }
            instruction_pointer += 1;
        }
    }

    fn binary_operation(&mut self, operator: InfixOperatorType) {
        let right = self.pop();
        let left = self.pop();
        match *right {
            Object::Int(r) => match *left {
                Object::Int(l) => {
                    let value = binary_integer_operation(operator, l, r);
                    self.push(Rc::new(Object::Int(value)));
                }

                _ => panic!("Don't know how to deal with {right:?} and {left:?} for {operator:?}"),
            },
            _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
        }
    }

    pub(crate) fn last_poped_stack_element(&self) -> Option<&Object> {
        self.stack.get(self.stack_pointer).map(|x| x.as_ref())
    }

    fn push(&mut self, object: Rc<Object>) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("stack overflow")
        }
        self.stack[self.stack_pointer] = object;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> Rc<Object> {
        if self.stack_pointer == 0 {
            panic!("Cannot move from stack, stack is empty")
        }
        let object = self.stack[self.stack_pointer - 1].clone();
        self.stack_pointer -= 1;
        return object;
    }
}

const CONSTANT: u8 = OpCodes::Constant as u8;
const ADD: u8 = OpCodes::Add as u8;
const MULTIPLY: u8 = OpCodes::Multiply as u8;
const DIVIDE: u8 = OpCodes::Divide as u8;
const SUBSTITUTE: u8 = OpCodes::Subtitute as u8;
const POP: u8 = OpCodes::Pop as u8;

const NIL: Object = Object::Null;

fn binary_integer_operation(operator: InfixOperatorType, left: i64, right: i64) -> i64 {
    match operator {
        InfixOperatorType::Plus => left + right,
        InfixOperatorType::Minus => left - right,
        InfixOperatorType::Multiply => left * right,
        InfixOperatorType::Divide => left / right,

        _ => panic!("Don't know how to deal with {right:?} and {left:?} for {operator:?}"),
    }
}
