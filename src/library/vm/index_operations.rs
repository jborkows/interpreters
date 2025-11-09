use std::rc::Rc;

use crate::{object::Object, vm::NIL};

pub(crate) fn execute_array_index(elements: Vec<Rc<Object>>, index_value: i64) -> Object {
    if index_value == 0 {
        NIL
    } else if index_value < 0 {
        let index = (-index_value) as usize;
        if index > elements.len() {
            NIL
        } else {
            let value = elements[elements.len() - index].clone();
            Rc::unwrap_or_clone(value)
        }
    } else {
        let index = index_value as usize;
        if index > elements.len() {
            NIL
        } else {
            let value = elements[index - 1].clone();
            Rc::unwrap_or_clone(value)
        }
    }
}
