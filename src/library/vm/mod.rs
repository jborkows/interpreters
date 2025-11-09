mod binary_operations;
mod index_operations;
#[cfg(test)]
mod testing;
mod vm;
pub use vm::VM;

use crate::object::Object;

pub(crate) fn wrap_boolean(value: bool) -> Object {
    match value {
        true => TRUE,
        false => FALSE,
    }
}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NIL: Object = Object::Null;
