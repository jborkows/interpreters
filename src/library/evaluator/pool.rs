use std::rc::Rc;

use crate::object::Object;

pub(super) const FALSE: Object = Object::Boolean(false);
pub(super) const TRUE: Object = Object::Boolean(true);
pub(super) const NULL: Object = Object::Null;

pub(super) fn string_value(string: String) -> Object {
    Object::String(string)
}

pub(super) fn int_value(int: i64) -> Object {
    if int >= 0 && int <= 255 {
        // Use the preallocated small integer pool for small integers
        // no real benefit looking at allocation counting here, but it is for fun of generating the
        // code
        return super::object_pool::SMALL_INTS[int as usize].clone();
    }
    Object::Int(int)
}
