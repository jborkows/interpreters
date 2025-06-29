use std::{cell::RefCell, rc::Rc};

use crate::{
    evaluator::evaluate, join_collection, object::Object, parser::Parser, print_bash_error,
};

#[macro_export]
macro_rules! should_be_equal_parsed {
    ($left:expr, $right:expr, $variant:ident) => {
        let left = eval_input($left);
        match *left {
            Object::$variant(value) => assert_eq!(
                value,
                $right,
                "Expected {} with value {}, but got {} for input {}",
                stringify!($variant),
                $right.to_string(),
                left.to_string(),
                $left.to_string()
            ),
            _ => panic!(
                concat!(
                    "Expected ",
                    stringify!($variant),
                    " with value {}, but got {}"
                ),
                $right.to_string(),
                left.to_string()
            ),
        };
    };
}

#[macro_export]
macro_rules! expected_string_to_be_equal {
    ($object:expr, $expected:expr) => {
        match $object.as_ref() {
            $crate::object::Object::String(value) => {
                assert_eq!(
                    value.to_string(),
                    $expected.to_string(),
                    "Expected String with value {}, but got {} for input {}",
                    $expected,
                    value,
                    $object.to_string()
                );
            }
            _ => panic!(
                "Expected a HashMap object, but got: {}",
                $object.to_string()
            ),
        }
    };
}
#[macro_export]
macro_rules! expected_boolean_object_to_be_equal {
    ($object:expr, $expected:expr) => {
        match $object.as_ref() {
            $crate::object::Object::Boolean(value) => {
                assert_eq!(
                    value,
                    &$expected,
                    "Expected Boolean with value {}, but got {} for input {}",
                    $expected,
                    value,
                    $object.to_string()
                );
            }
            _ => panic!(
                "Expected a HashMap object, but got: {}",
                $object.to_string()
            ),
        }
    };
}

#[macro_export]
macro_rules! expected_integer_value_to_be {
    ($object:expr, $expected:expr) => {
        match $object {
            $crate::object::Object::Int(value) => {
                assert_eq!(
                    value,
                    $expected,
                    "Expected Int with value {}, but got {} for input {}",
                    $expected,
                    value,
                    $object.to_string()
                );
            }
            _ => panic!(
                "Expected a HashMap object, but got: {}",
                $object.to_string()
            ),
        }
    };
}

#[macro_export]
macro_rules! expected_integer_as_result_tests {
    ($($name:ident: ($input:expr, $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                 crate::evaluator::tests::evaluator_tests::should_be_integer_equal_to($input, $expected);
            }
        )*
    };
}
#[macro_export]
macro_rules! expected_error_with_text {
    ($expected:expr, { $($name:ident: $input:expr,)* }) => {
        $(
            #[test]
            fn $name() {
                crate::evaluator::tests::evaluator_tests::should_be_error_with_text($input, $expected);
            }
        )*
    };
}

pub(super) fn eval_input(input: &str) -> Rc<Object> {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    let env = Rc::new(RefCell::new(crate::object::Environment::new()));
    evaluate(&program, env)
}

pub(super) fn should_be_integer_equal_to(left: &str, right: i64) {
    should_be_equal_parsed!(left, right, Int);
}

pub(super) fn should_be_boolean_equal_to(left: &str, right: bool) {
    should_be_equal_parsed!(left, right, Boolean);
}

pub(super) fn should_be_string_equal_to(left: &str, right: String) {
    let left = eval_input(left);
    let left_string = left.to_string();
    match left.as_ref() {
        Object::String(value) => assert_eq!(
            value,
            &right,
            "Expected String with value {}, but got {} for input {}",
            right.to_string(),
            left_string,
            left_string
        ),
        _ => panic!(
            "Expected String with value {}, but got {} for input {}",
            right.to_string(),
            left.to_string(),
            left.to_string()
        ),
    };
}

pub(super) fn should_be_null(left: &str) {
    let left = eval_input(left);
    match *left {
        Object::Null => {}
        _ => panic!(
            "Expected Null , but got {} for input {}",
            left.to_string(),
            left.to_string()
        ),
    };
}

pub(super) fn should_be_error(left_input: &str) {
    let left = eval_input(left_input);
    match *left {
        Object::Error { .. } => {}
        _ => panic!(
            "Expected Error, but got {} for input {}",
            left.to_string(),
            left_input.to_string()
        ),
    };
}
pub(super) fn should_be_error_with_text(left_input: &str, error_text: &str) {
    let left = eval_input(left_input);
    match left.as_ref() {
        Object::Error {
            message,
            line: _,
            column: _,
        } => {
            assert!(
                message.contains(error_text),
                "Expected error message to contain '{}', but got '{}' for input {}",
                error_text,
                message,
                left_input.to_string()
            );
        }
        _ => panic!(
            "Expected Error, but got {} for input {}",
            left.to_string(),
            left_input.to_string()
        ),
    };
}

pub(super) fn check_parser_errors(parser: &Parser) {
    if !parser.errors().is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors(), "\n"))
        );
    }
}
