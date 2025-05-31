use std::rc::Rc;

use crate::object::Object;

const FALSE: Object = Object::Boolean(false);
const TRUE: Object = Object::Boolean(true);
const NULL: Object = Object::Null;

pub fn string_value(string: String) -> Rc<Object> {
    Rc::new(Object::String(string))
}
pub fn null_value() -> Rc<Object> {
    Rc::new(NULL)
}

pub fn int_value(int: i64) -> Rc<Object> {
    if int >= 0 && int <= 255 {
        // Use the preallocated small integer pool for small integers
        // no real benefit looking at allocation counting here, but it is for fun of generating the
        // code
        return Rc::new(super::object_pool::SMALL_INTS[int as usize].clone());
    }
    Rc::new(Object::Int(int))
}

pub fn false_value() -> Rc<Object> {
    Rc::new(FALSE)
}
pub fn true_value() -> Rc<Object> {
    Rc::new(TRUE)
}
pub fn boolean_value(value: bool) -> Rc<Object> {
    if value { Rc::new(TRUE) } else { Rc::new(FALSE) }
}
pub fn is_truthy(condition_value: &Object) -> bool {
    if *condition_value == NULL {
        return false;
    }
    if *condition_value == FALSE {
        return false;
    }
    return true;
}
