use std::rc::Rc;

use crate::object::Object;
use crate::object::hash;

macro_rules! should_be_same_cache {
    ($($name:ident: ($input:expr, $other:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let input_hash = hash($input);
                let other_hash = hash($other);
                assert_eq!(
                hash($input),
                hash($other),
                "Expected same hash for input: {}({}) and {}({}), but got different hashes",
                $input, input_hash,$other, other_hash
            );
            }
        )*
    };
}
macro_rules! should_be_different_cache {
    ($($name:ident: ($input:expr, $other:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let input_hash = hash($input);
                let other_hash = hash($other);
                assert_ne!(
                hash($input),
                hash($other),
                "Expected differrent hash for input: {}({}) and {}({}), but got same hashes",
                $input, input_hash,$other, other_hash
            );
            }
        )*
    };
}

should_be_same_cache! {
    same_string: (&Object::String("a".to_string()),  Rc::new(Object::String("a".to_string())).as_ref()),
    same_integer: (&Object::Int(42), Rc::new(Object::Int(42)).as_ref()),
    same_larger_integer: (&Object::Int(1000), Rc::new(Object::Int(1000)).as_ref()),
    same_boolean_true: (&Object::Boolean(true), Rc::new(Object::Boolean(true)).as_ref()),
    same_boolean_false: (&Object::Boolean(false), Rc::new(Object::Boolean(false)).as_ref()),
}

should_be_different_cache! {
    different_string: (&Object::String("b".to_string()),  Rc::new(Object::String("a".to_string())).as_ref()),
    different_integer: (&Object::Int(43), Rc::new(Object::Int(42)).as_ref()),
    different_larger_integer: (&Object::Int(1001), Rc::new(Object::Int(1000)).as_ref()),
    different_false_true: (&Object::Boolean(false), &Object::Boolean(true)),
    different_false_and_0: (&Object::Boolean(false), &Object::Int(0)),
    different_true_and_1: (&Object::Boolean(true), &Object::Int(1)),
    different_null_and_0: (&Object::Null, &Object::Int(0)),

}
