use crate::code::definitions::Definition;
use std::collections::HashMap;

use std::sync::LazyLock;

mod compiler;
mod definitions;
mod make;
mod symbol_table;
#[cfg(test)]
mod testing;

pub use compiler::{Bytecode, CompilationError, compile};
pub use definitions::Byte;
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
        pair(OpCodes::False, vec![]),
        pair(OpCodes::True, vec![]),
        pair(OpCodes::Equal, vec![]),
        pair(OpCodes::NotEqual, vec![]),
        pair(OpCodes::GreaterThan, vec![]),
        pair(OpCodes::Bang, vec![]),
        pair(OpCodes::Minus, vec![]),
        pair(OpCodes::Jump, vec![2]),
        pair(OpCodes::JumpNotTruthy, vec![2]),
        pair(OpCodes::Null, vec![]),
        pair(OpCodes::SetGlobal, vec![2]),
        pair(OpCodes::GetGlobal, vec![2]),
        pair(OpCodes::Array, vec![2]), //operand number of elements in array
        pair(OpCodes::Hash, vec![2]),  //operand number of keys and values from map
        pair(OpCodes::Index, vec![]),
        pair(OpCodes::Call, vec![1]), //number of arguments passed
        pair(OpCodes::ReturnValue, vec![]),
        pair(OpCodes::ReturnNone, vec![]),
        pair(OpCodes::SetLocal, vec![1]),
        pair(OpCodes::GetLocal, vec![1]),
        pair(OpCodes::GetBuiltin, vec![1]),
        pair(OpCodes::Closure, vec![2, 1]), //[constant index of compiled function, number of free
        //variables in function]
        pair(OpCodes::GetFree, vec![1]),
        pair(OpCodes::CurrentClosure, vec![]),
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
pub fn read_u_8(entry: &[Byte]) -> u16 {
    entry[0].0 as u16
}
