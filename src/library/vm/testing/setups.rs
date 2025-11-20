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
pub(crate) fn should_be_string(value: &str) -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::String(v) => assert_eq!(&value, v, "Expecing {:?} got {:?}", value, v),
        _ => panic!("Expecting int got {:?}", object),
    }
}
pub(crate) fn should_be_boolean(value: bool) -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::Boolean(v) => assert_eq!(&value, v, "Expecing {:?} got {:?}", value, v),
        _ => panic!("Expecting boolean got {:?}", object),
    }
}
pub(crate) fn should_be_error<'a, F>(message_checker: F) -> impl Fn(&Object)
where
    F: Fn(&str) -> Result<(), &'a str> + 'a,
{
    move |object: &Object| match object {
        Object::Error { message, .. } => {
            let result = message_checker(&message);
            if let Err(error_message) = result {
                panic!("For error {message} got {error_message}");
            }
        }
        _ => panic!("Expecting error got {:?}", object),
    }
}
pub(crate) fn should_be_null() -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::Null => {}
        _ => panic!("Expecting Null got {:?}", object),
    }
}

pub(crate) fn should_be_integer_array(expected: &[i64]) -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::Array { elements } => {
            assert_eq!(
                expected.len(),
                elements.len(),
                "Expecting {expected:?} got {elements:?}"
            );
            for (e, a) in expected.iter().zip(elements) {
                should_be_integer(e.clone())(a.as_ref());
            }
        }
        _ => panic!("Expecting {expected:?} got {:?}", object),
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
    checker(&stack_element);
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
