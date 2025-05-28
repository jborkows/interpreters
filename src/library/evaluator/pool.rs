use crate::object::Object;

pub(super) const FALSE: Object = Object::Boolean(false);
pub(super) const TRUE: Object = Object::Boolean(true);
pub(super) const NULL: Object = Object::Null;
