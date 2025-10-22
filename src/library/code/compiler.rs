use std::vec;

use crate::{
    ast::base::Node,
    code::definitions::{Byte, Instructions},
    object::Object,
};

#[derive(Debug)]
pub enum CompilationError {}

pub fn compile<T: Node>(node: T) -> Result<Bytecode, CompilationError> {
    return Result::Ok(Bytecode {
        instructions: Instructions(vec![Byte(0); 6]),
        constants: vec![],
    });
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
