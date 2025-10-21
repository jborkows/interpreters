use crate::code::definitions::{Byte, Definition, OpCode, OpCodes};
use std::collections::HashMap;

use std::sync::LazyLock;

mod compiler;
mod definitions;
mod make;
#[cfg(test)]
mod testing;

static DEFINITIONS: LazyLock<HashMap<OpCode, Definition>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        OpCode(Byte(OpCodes::Constant as u8)),
        Definition {
            name: "OpConstant".to_string(),
            operands_widths: vec![2],
        },
    );
    m
});

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
