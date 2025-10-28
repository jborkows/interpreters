use crate::code::definitions::{Byte, Definition};
use std::collections::HashMap;

use std::sync::LazyLock;

mod compiler;
mod definitions;
mod make;
#[cfg(test)]
mod testing;

pub use compiler::{Bytecode, CompilationError, compile};
pub use definitions::Instructions;
pub use definitions::OpCode;
pub use definitions::OpCodes;

static DEFINITIONS: LazyLock<HashMap<OpCode, Definition>> = LazyLock::new(|| {
    return HashMap::from([
        pair(OpCodes::Constant, vec![2]),
        pair(OpCodes::Add, vec![]),
        pair(OpCodes::Pop, vec![]),
        pair(OpCodes::Multiply, vec![]),
        pair(OpCodes::Subtitute, vec![]),
        pair(OpCodes::Divide, vec![]),
    ]);
});

fn pair(op_code: OpCodes, operand_widths: Vec<usize>) -> (OpCode, Definition) {
    (
        op_code.into(),
        Definition {
            name: op_code.to_string(),
            operands_widths: operand_widths,
        },
    )
}

#[derive(Debug)]
pub enum LookupError {
    OpCodeNotFound,
}
pub fn lookup<'a>(op_code: &OpCode) -> Result<&'a Definition, LookupError> {
    match DEFINITIONS.get(op_code) {
        Some(v) => Ok(v),
        None => Result::Err(LookupError::OpCodeNotFound),
    }
}

pub fn read_u_16(entry: &[Byte]) -> u16 {
    (entry[0].0 as u16) * 256 + (entry[1].0 as u16)
}
