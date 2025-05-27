use crate::{join_collection, object::Object, parser::Parser, print_bash_error};

use super::evaluate;

#[test]
fn text_evalaution_of_integers() {
    should_be_integer_equal_to(eval_input("1"), 1);
    should_be_integer_equal_to(eval_input("3"), 3);
}

fn eval_input(input: &str) -> Object {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    evaluate(&program)
}

fn should_be_integer_equal_to(left: Object, right: i64) {
    match left {
        Object::Int(value) => assert_eq!(value, right),
        _ => panic!("Expected an integer, got: {}", left.to_string()),
    }
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors().is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors(), "\n"))
        );
    }
}
