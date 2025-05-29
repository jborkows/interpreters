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
    should_be_integer_equal_to(eval_input("0"), 0);
    should_be_integer_equal_to(eval_input("1"), 1);
    should_be_integer_equal_to(eval_input("3"), 3);
}

#[test]
fn text_evaluation_of_booleans() {
    should_be_boolean_equal_to(eval_input("true"), true);
    should_be_boolean_equal_to(eval_input("false"), false);
}

#[test]
fn text_evaluation_of_strings() {
    should_be_string_equal_to(eval_input("\"Hello, World!\""), "Hello, World!".to_string());
    should_be_string_equal_to(eval_input("\"Test String\""), "Test String".to_string());
}

#[test]
fn test_bang_operator() {
    should_parsed_be_boolean_equal_to("!\"\"", true);
    should_parsed_be_boolean_equal_to("!\"  \"", true);
    should_parsed_be_boolean_equal_to("!\"xx\"", false);
    should_parsed_be_boolean_equal_to("!!\"xx\"", true);
    should_parsed_be_boolean_equal_to("!5", false);
    should_parsed_be_boolean_equal_to("!0", false);
    should_parsed_be_boolean_equal_to("!!0", true);
    should_parsed_be_boolean_equal_to("!!5", true);
    should_parsed_be_boolean_equal_to("!!true", true);
    should_parsed_be_boolean_equal_to("!!false", false);
}

fn eval_input(input: &str) -> Object {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    evaluate(&program)
}

fn should_be_integer_equal_to(left: Object, right: i64) {
    should_be_equal!(left, right, Int);
}

fn should_parsed_be_boolean_equal_to(left: &str, right: bool) {
    should_be_equal_parsed!(left, right, Boolean);
}

fn should_be_boolean_equal_to(left: Object, right: bool) {
    should_be_equal!(left, right, Boolean);
}

fn should_be_string_equal_to(left: Object, right: String) {
    should_be_equal!(left, right, String);
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors().is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors(), "\n"))
        );
    }
}
