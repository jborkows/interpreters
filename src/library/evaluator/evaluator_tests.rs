use crate::{
    current_allocation_counting, join_collection, object::Object, parser::Parser, print_bash_error,
};

use super::evaluate;

macro_rules! should_be_equal {
    ($left:expr, $right:ident, $variant:ident) => {
        match $left {
            Object::$variant(value) => assert_eq!(value, $right),
            _ => panic!(
                concat!(
                    "Expected ",
                    stringify!($variant),
                    " with value {}, but got {}"
                ),
                $right.to_string(),
                $left.to_string()
            ),
        };
    };
}
macro_rules! should_be_equal_parsed {
    ($left:expr, $right:ident, $variant:ident) => {
        let left = eval_input($left);
        match left {
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
                $left.to_string()
            ),
        };
    };
}

#[test]
fn text_evalaution_of_integers() {
    current_allocation_counting!();
    should_be_integer_equal_to("0", 0);
    should_be_integer_equal_to("1", 1);
    should_be_integer_equal_to("3", 3);
}

#[test]
fn text_evaluation_of_booleans() {
    should_be_boolean_equal_to("true", true);
    should_be_boolean_equal_to("false", false);
}

#[test]
fn text_evaluation_of_strings() {
    should_be_string_equal_to("\"Hello, World!\"", "Hello, World!".to_string());
    should_be_string_equal_to("\"Test String\"", "Test String".to_string());
}

#[test]
fn test_bang_operator() {
    should_be_boolean_equal_to("!\"\"", true);
    should_be_boolean_equal_to("!\"  \"", true);
    should_be_boolean_equal_to("!\"xx\"", false);
    should_be_boolean_equal_to("!!\"xx\"", true);
    should_be_boolean_equal_to("!5", false);
    should_be_boolean_equal_to("!0", false);
    should_be_boolean_equal_to("!!0", true);
    should_be_boolean_equal_to("!!5", true);
    should_be_boolean_equal_to("!!true", true);
    should_be_boolean_equal_to("!!false", false);
}

#[test]
fn test_minus_operator() {
    should_be_integer_equal_to("-5", -5);
    should_be_integer_equal_to("-10", -10);
}

#[test]
fn test_integer_calculations() {
    should_be_integer_equal_to("5 + 5", 10);
    should_be_integer_equal_to("10 - 5", 5);
    should_be_integer_equal_to("2 * 3", 6);
    should_be_integer_equal_to("8 / 2", 4);
    should_be_integer_equal_to("2 + 3 * 4", 14);
    should_be_integer_equal_to("(2 + 3) * 4", 20);
    should_be_integer_equal_to("-5 + 10 -5", 0);
}

#[test]
fn test_string_calculations() {
    should_be_string_equal_to("\"x\"+\"y\"", "xy".to_string());
    should_be_string_equal_to("\"x\"*3", "xxx".to_string());
    should_be_string_equal_to("\"x\"+3", "x3".to_string());
    should_be_string_equal_to("3+\"x\"", "3x".to_string());
}

fn eval_input(input: &str) -> Object {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    evaluate(&program)
}

fn should_be_integer_equal_to(left: &str, right: i64) {
    should_be_equal_parsed!(left, right, Int);
}

fn should_be_boolean_equal_to(left: &str, right: bool) {
    should_be_equal_parsed!(left, right, Boolean);
}

fn should_be_string_equal_to(left: &str, right: String) {
    let left = eval_input(left);
    let left_string = left.to_string();
    match left {
        Object::String(value) => assert_eq!(
            value,
            right,
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

fn check_parser_errors(parser: &Parser) {
    if !parser.errors().is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors(), "\n"))
        );
    }
}
