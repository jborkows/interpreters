use crate::{
    object::{HashEntry, HashValue, hash},
    vm::{FALSE, TRUE, wrap_boolean},
};
use std::{collections::HashMap, panic, rc::Rc};

use crate::{
    ast::expression::{InfixOperatorType, PrefixOperatorType},
    code::{Bytecode, Instructions, OpCodes, read_u_16},
    object::{Object, is_truthy},
    vm::binary_operations::binary,
};

const STACK_SIZE: usize = 2048;
#[cfg(not(test))]
const GLOBALS_SIZE: usize = 0xFFFF;
#[cfg(test)]
const GLOBALS_SIZE: usize = 0xFF;
pub struct VM {
    constants: Vec<Object>,
    instructions: Instructions,
    stack: [Object; STACK_SIZE],
    stack_pointer: usize, //points to the next value. Top of stack is stack[stack_pointer-1]
    globals: [Object; GLOBALS_SIZE],
}

impl VM {
    pub fn new(byte_code: Bytecode) -> Self {
        let constants: Vec<Object> = byte_code
            .constants
            .iter()
            .map(|o| o.clone())
            .collect::<Vec<_>>();
        VM {
            constants: constants,
            instructions: byte_code.instructions,
            stack: std::array::from_fn(|_| NIL),
            stack_pointer: 0,
            globals: std::array::from_fn(|_| NIL),
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
                EQUAL => {
                    self.binary_operation(InfixOperatorType::Equal);
                }
                NOT_EQUAL => {
                    self.binary_operation(InfixOperatorType::NotEqual);
                }
                GRATER => {
                    self.binary_operation(InfixOperatorType::GreaterThan);
                }
                POP => {
                    self.pop();
                }
                TRUE_OP => {
                    self.push(TRUE);
                }
                FALSE_OP => {
                    self.push(FALSE);
                }
                MINUS => self.prefix_operation(PrefixOperatorType::Minus),
                BANG => self.prefix_operation(PrefixOperatorType::Bang),
                JUMP => {
                    let position = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer = position - 1; //not to position, it will be incremented
                    //at end of loop
                }
                JUMP_NOT_TRUTHY => {
                    let position = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer += 2;
                    let condition = self.pop();
                    if !is_truthy(&condition) {
                        instruction_pointer = position - 1; //same as with jump
                    }
                }
                NULL_OP => self.push(NIL),
                SET_GLOBAL => {
                    let global_index = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer += 2;
                    self.globals[global_index] = self.pop()
                }
                GET_GLOBAL => {
                    let global_index = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer += 2;
                    let value = self.globals[global_index].clone();
                    self.push(value);
                }
                ARRAY => {
                    let number_of_elements = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer += 2;
                    let array = self
                        .build_array(self.stack_pointer - number_of_elements, self.stack_pointer);
                    self.stack_pointer = self.stack_pointer - number_of_elements;
                    self.push(array);
                }
                HASH => {
                    let number_of_elements = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    instruction_pointer += 2;
                    let maps =
                        self.build_map(self.stack_pointer - number_of_elements, self.stack_pointer);
                    self.stack_pointer = self.stack_pointer - number_of_elements;
                    self.push(maps);
                }
                _ => panic!("Don't know what to do with {instruction}"),
            }
            instruction_pointer += 1;
        }
    }

    fn binary_operation(&mut self, operator: InfixOperatorType) {
        let right = self.pop();
        let left = self.pop();
        let value = binary(left, right, operator);
        self.push(value)
    }

    fn prefix_operation(&mut self, operator: PrefixOperatorType) {
        let right = self.pop();
        match right {
            Object::Int(r) => {
                let operator = operator;
                match operator {
                    PrefixOperatorType::Minus => self.push(Object::Int(-r)),
                    _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
                };
            }
            Object::Boolean(r) => {
                let object = match operator {
                    PrefixOperatorType::Bang => wrap_boolean(!r),
                    _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
                };
                self.push(object);
            }
            Object::Null => {
                match operator {
                    PrefixOperatorType::Bang => self.push(TRUE),
                    _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
                };
            }
            _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
        }
    }

    pub(crate) fn last_poped_stack_element(&self) -> Option<Object> {
        self.stack.get(self.stack_pointer).map(|o| o.clone())
    }

    fn push(&mut self, object: Object) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("stack overflow")
        }
        self.stack[self.stack_pointer] = object;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> Object {
        if self.stack_pointer == 0 {
            panic!("Cannot move from stack, stack is empty")
        }
        let object = self.stack[self.stack_pointer - 1].clone();
        self.stack_pointer -= 1;
        return object.clone();
    }

    fn build_array(&mut self, start_index: usize, end_index: usize) -> Object {
        if start_index == end_index {
            return Object::Array { elements: vec![] };
        }
        let mut elements: Vec<Rc<Object>> = Vec::with_capacity(end_index - start_index);
        let mut i = start_index;
        while i < end_index {
            elements.push(Rc::new(self.stack[i].clone()));
            i += 1;
        }
        return Object::Array { elements };
    }

    fn build_map(&self, start_index: usize, end_index: usize) -> Object {
        if start_index == end_index {
            return Object::HashMap(HashMap::new());
        }
        let mut elements: HashMap<HashValue, Rc<HashEntry>> =
            HashMap::with_capacity(end_index - start_index);
        let mut i = start_index;
        while i < end_index {
            let key = self.stack[i].clone();
            let value = self.stack[i + 1].clone();
            elements.insert(
                hash(&key),
                Rc::new(HashEntry {
                    key: Rc::new(key),
                    value: Rc::new(value),
                }),
            );
            i += 2;
        }
        return Object::HashMap(elements);
    }
}

const CONSTANT: u8 = OpCodes::Constant as u8;
const ADD: u8 = OpCodes::Add as u8;
const MULTIPLY: u8 = OpCodes::Multiply as u8;
const DIVIDE: u8 = OpCodes::Divide as u8;
const SUBSTITUTE: u8 = OpCodes::Subtitute as u8;
const POP: u8 = OpCodes::Pop as u8;
const BANG: u8 = OpCodes::Bang as u8;
const MINUS: u8 = OpCodes::Minus as u8;
const JUMP: u8 = OpCodes::Jump as u8;
const JUMP_NOT_TRUTHY: u8 = OpCodes::JumpNotTruthy as u8;
const NULL_OP: u8 = OpCodes::Null as u8;
const ARRAY: u8 = OpCodes::Array as u8;
const HASH: u8 = OpCodes::Hash as u8;

const GET_GLOBAL: u8 = OpCodes::GetGlobal as u8;
const SET_GLOBAL: u8 = OpCodes::SetGlobal as u8;
const NIL: Object = Object::Null;
const TRUE_OP: u8 = OpCodes::True as u8;
const FALSE_OP: u8 = OpCodes::False as u8;
const GRATER: u8 = OpCodes::GreaterThan as u8;
const EQUAL: u8 = OpCodes::Equal as u8;
const NOT_EQUAL: u8 = OpCodes::NotEqual as u8;
