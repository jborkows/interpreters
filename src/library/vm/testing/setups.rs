use std::panic;

use crate::{
    ast::statements::Program, code::compile, join_collection, object::Object, parser::Parser,
    print_bash_error, vm::vm::VM,
};

pub(crate) fn parse_program(input: &str) -> Program {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    program
}

pub(crate) fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors, "\n"))
        );
    }
}

pub(crate) fn should_be_integer(value: i64) -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::Int(v) => assert_eq!(&value, v, "Expecing {:?} got {:?}", value, v),
        _ => panic!("Expecting int got {:?}", object),
    }
}

pub(crate) fn run_vm_test(input: &str, checker: impl Fn(&Object)) {
    let program = parse_program(input);
    let byte_code = match compile(program) {
        Ok(v) => v,
        Err(e) => {
            for error in e {
                println!("Found error: {:?}", error)
            }
            panic!("Compilation failed")
        }
    };
    let mut vm = VM::new(byte_code);
    vm.run();
    let stack_element = match vm.last_poped_stack_element() {
        Some(v) => v,
        None => panic!("No object on stack"),
    };
    checker(stack_element);
}
#[macro_export]
macro_rules! generate_vm_tests {
    ($($name:ident: ($program_text:expr, $checker:expr),)*) => {
        $(
            #[test]
            fn $name() {
                run_vm_test($program_text, $checker);
            }
        )*
    };

}
