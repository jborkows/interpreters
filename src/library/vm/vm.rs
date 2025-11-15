use crate::{
    code::read_u_8,
    object::{HashEntry, HashValue, hash},
    vm::{
        FALSE, NIL, TRUE,
        frame::{Frame, NIL_FRAME},
        index_operations::execute_array_index,
        wrap_boolean,
    },
};
use std::{collections::HashMap, panic, rc::Rc};

use crate::{
    ast::expression::{InfixOperatorType, PrefixOperatorType},
    code::{Bytecode, OpCodes, read_u_16},
    object::{Object, is_truthy},
    vm::binary_operations::binary,
};

const STACK_SIZE: usize = 2048;
#[cfg(not(test))]
const GLOBALS_SIZE: usize = 0xFFFF;
#[cfg(test)]
const GLOBALS_SIZE: usize = 0xFF;
#[cfg(not(test))]
const FRAME_SIZE: usize = 1024;
#[cfg(test)]
const FRAME_SIZE: usize = 100;
pub struct VM {
    constants: Vec<Object>,
    stack: [Object; STACK_SIZE],
    stack_pointer: usize, //points to the next value. Top of stack is stack[stack_pointer-1]
    globals: [Object; GLOBALS_SIZE],
    frames: [Frame; FRAME_SIZE],
    frame_index: usize,
}

impl VM {
    pub fn new(byte_code: Bytecode) -> Self {
        #[cfg(test)]
        println!("Processing\n{:?}", byte_code.instructions);
        let constants: Vec<Object> = byte_code
            .constants
            .iter()
            .map(|o| o.clone())
            .collect::<Vec<_>>();

        let mut vm = VM {
            constants: constants,
            stack: std::array::from_fn(|_| NIL),
            stack_pointer: 0,
            globals: std::array::from_fn(|_| NIL),
            frames: std::array::from_fn(|_| NIL_FRAME),
            frame_index: 0,
        };
        vm.push_frame(Frame::new(byte_code.instructions, 0));
        vm
    }

    pub fn run(&mut self) {
        let mut move_instruction_pointer: usize;
        while self.current_frame().instruction_pointer < self.current_frame().function.bytes().len()
        {
            let instruction_pointer = self.current_frame().instruction_pointer;
            let bytes = self.current_frame().function.bytes();
            let instruction: u8 = bytes.get(instruction_pointer).unwrap().into();
            move_instruction_pointer = 1;
            #[cfg(test)]
            debug(instruction);
            match instruction {
                CONSTANT => {
                    let constant_index = read_u_16(&bytes[instruction_pointer + 1..]);
                    self.current_frame().instruction_pointer += 2;
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
                    self.current_frame().instruction_pointer = position - 1; //not to position, it will be incremented
                    //at end of loop
                }
                JUMP_NOT_TRUTHY => {
                    let position = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 2;
                    let condition = self.pop();
                    if !is_truthy(&condition) {
                        self.current_frame().instruction_pointer = position - 1; //same as with jump
                    }
                }
                NULL_OP => self.push(NIL),
                SET_GLOBAL => {
                    let global_index = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 2;
                    self.globals[global_index] = self.pop()
                }
                GET_GLOBAL => {
                    let global_index = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 2;
                    let value = self.globals[global_index].clone();
                    self.push(value);
                }
                ARRAY => {
                    let number_of_elements = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 2;
                    let array = self
                        .build_array(self.stack_pointer - number_of_elements, self.stack_pointer);
                    self.stack_pointer = self.stack_pointer - number_of_elements;
                    self.push(array);
                }
                HASH => {
                    let number_of_elements = read_u_16(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 2;
                    let maps =
                        self.build_map(self.stack_pointer - number_of_elements, self.stack_pointer);
                    self.stack_pointer = self.stack_pointer - number_of_elements;
                    self.push(maps);
                }
                INDEX => {
                    let index = self.pop();
                    let left = self.pop();
                    self.execute_index(index, left);
                }
                RETURN_VALUE => {
                    let frame = self.pop_frame();
                    let value = self.pop();
                    self.stack_pointer = frame.base_pointer - 1;
                    self.push(value);
                }
                NO_RETURN => {
                    let frame = self.pop_frame();
                    self.stack_pointer = frame.base_pointer - 1;
                    self.push(NIL)
                }
                SET_LOCAL => {
                    let local_index = read_u_8(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 1;
                    let index = self.current_frame().base_pointer + local_index;
                    self.stack[index] = self.pop();
                }
                GET_LOCAL => {
                    let local_index = read_u_8(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 1;
                    let index = self.current_frame().base_pointer + local_index;
                    let object = self.stack[index].clone();
                    self.push(object);
                }
                CALL => {
                    let number_of_arguments = read_u_8(&bytes[instruction_pointer + 1..]) as usize;
                    self.current_frame().instruction_pointer += 1;
                    self.call_function(number_of_arguments);
                    move_instruction_pointer = 0;
                }
                _ => panic!("Don't know what to do with {instruction}"),
            }
            self.current_frame().instruction_pointer += move_instruction_pointer;
        }
    }

    fn binary_operation(&mut self, operator: InfixOperatorType) {
        let right = self.pop();
        let left = self.pop();
        let value = binary(left, right, operator);
        self.push(value);
    }

    fn current_frame(&mut self) -> &mut Frame {
        self.frames
            .get_mut(self.frame_index - 1)
            .expect("Has to find frame")
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames[self.frame_index] = frame;
        self.frame_index += 1;
    }

    fn pop_frame(&mut self) -> Frame {
        let frame = self.frames[self.frame_index - 1].clone();
        self.frames[self.frame_index - 1] = NIL_FRAME;
        self.frame_index -= 1;
        frame
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

    fn relative_stack_down(&mut self, possition: usize) -> Object {
        if self.stack_pointer < possition {
            panic!(
                "Cannot get from stack at {possition} since stack pointer is {}",
                self.stack_pointer
            )
        }
        let object = self.stack[self.stack_pointer - 1 - possition].clone();
        return object.clone();
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

    fn execute_index(&mut self, index: Object, left: Object) {
        match left {
            Object::Array { elements } => {
                let index_value = match index {
                    Object::Int(v) => v,
                    _ => panic!("Cannot do index {index:?} operation on array"),
                };

                self.push(execute_array_index(elements, index_value));
            }
            Object::HashMap(hash_map) => {
                let hash = hash(&index);
                let value = hash_map.get(&hash);
                let object = match value {
                    Some(entry) => Rc::unwrap_or_clone(entry.value.clone()),
                    None => NIL,
                };
                self.push(object);
            }
            _ => panic!("Cannot do index operation on {left:?}"),
        }
    }

    fn call_function(&mut self, number_of_arguments: usize) {
        let object = self.relative_stack_down(number_of_arguments);
        match object {
            Object::CompiledFunction {
                instructions,
                number_of_locals,
            } => {
                let frame = Frame::new(instructions, self.stack_pointer - number_of_arguments);
                let instruction_pointer_position = frame.base_pointer + number_of_locals;
                self.push_frame(frame);
                self.stack_pointer = instruction_pointer_position;
            }
            _ => panic!("Can only call compiled function called {object}"),
        }
    }
}

fn debug(opcode: u8) {
    let text = match opcode {
        CONSTANT => "CONSTANT",
        ADD => "ADD",
        SUBSTITUTE => "SUBSTITUTE",
        MULTIPLY => "MULTIPLY",
        DIVIDE => "DIVIDE",
        EQUAL => "EQUAL",
        NOT_EQUAL => "NOT_EQUAL",
        GRATER => "GRATER",
        POP => "POP",
        TRUE_OP => "TRUE",
        FALSE_OP => "FALSE",
        MINUS => "MINUS",
        BANG => "BANG",
        JUMP => "JUMP",
        JUMP_NOT_TRUTHY => "JUMP_NOT_TRUTHY",
        NULL_OP => "NULL",
        SET_GLOBAL => "SET_GLOBAL",
        GET_GLOBAL => "GET_GLOBAL",
        ARRAY => "ARRAY",
        HASH => "HASH",
        INDEX => "INDEX",
        CALL => "CALL",
        RETURN_VALUE => "RETURN_VALUE",
        NO_RETURN => "NO_RETURN",
        SET_LOCAL => "SET_LOCAL",
        GET_LOCAL => "GET_LOCAL",
        _ => "NOT KNOW",
    };
    println!("{text}")
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
const INDEX: u8 = OpCodes::Index as u8;
const CALL: u8 = OpCodes::Call as u8;
const NO_RETURN: u8 = OpCodes::ReturnNone as u8;
const RETURN_VALUE: u8 = OpCodes::ReturnValue as u8;

const GET_GLOBAL: u8 = OpCodes::GetGlobal as u8;
const SET_GLOBAL: u8 = OpCodes::SetGlobal as u8;
const TRUE_OP: u8 = OpCodes::True as u8;
const FALSE_OP: u8 = OpCodes::False as u8;
const GRATER: u8 = OpCodes::GreaterThan as u8;
const EQUAL: u8 = OpCodes::Equal as u8;
const NOT_EQUAL: u8 = OpCodes::NotEqual as u8;
const GET_LOCAL: u8 = OpCodes::GetLocal as u8;
const SET_LOCAL: u8 = OpCodes::SetLocal as u8;
