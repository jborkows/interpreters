use crate::object::Object;

pub(super) const FALSE: Object = Object::Boolean(false);
pub(super) const TRUE: Object = Object::Boolean(true);
pub(super) const NULL: Object = Object::Null;

pub(super) fn string_value(string: String) -> Object {
    Object::String(string)
}

pub(super) fn int_value(int: i64) -> Object {
    Object::Int(int)
}
