use core::panic;

use std::fmt::{Debug, Display};

use crate::{
    ast::statements::Program,
    code::{
        compiler::compile,
        definitions::{Byte, Instructions},
    },
    join_collection,
    object::Object,
    parser::Parser,
    print_bash_error,
};

#[macro_export]
macro_rules! generate_tests_for_compiler {

    ($($name:ident: ($program_text:expr,$expected_instructions:expr,  $constant_checkers:expr),)*) => {
        $(
            #[test]
            fn $name() {
                test_compilation($program_text, $expected_instructions, $constant_checkers);
            }
        )*
    };

}
pub(crate) fn test_compilation(
    input: &str,
    expected_instructions: Vec<Instructions>,
    checkers: Vec<Box<dyn Fn(&Object, Index)>>,
) {
    let program = parse_program(input);
    let compiled = compile(program);
    let bytecode = match compiled {
        Ok(v) => v,
        Err(e) => panic!("Compilation errors: {:?}", e),
    };

    test_instructions(InstructionTest {
        expected: expected_instructions,
        actual: bytecode.instructions,
    });

    test_constants(checkers, bytecode.constants);
}

pub(crate) fn parse_program(input: &str) -> Program {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    program
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors, "\n"))
        );
    }
}

pub(crate) struct InstructionTest {
    expected: Vec<Instructions>,
    actual: Instructions,
}

pub(crate) fn test_instructions(input: InstructionTest) {
    let expected = concat_instructions(&input.expected);
    if expected.length() != input.actual.length() {
        panic!(
            "Expected instructions:\n{expected}\nactual was:\n{}",
            input.actual
        )
    }

    for (index, value) in expected
        .bytes()
        .iter()
        .zip(input.actual.bytes())
        .enumerate()
    {
        let (expected_byte, actual) = value;
        assert_eq!(
            expected_byte, &actual,
            "Expected {:?} but got {:?} at {:?}\nExpected instructions:\n{expected}\nactual was:\n{}",
            expected_byte, actual, index, input.actual
        );
    }
}

pub(crate) fn concat_instructions(instructions: &Vec<Instructions>) -> Instructions {
    let bytes: Vec<Byte> = instructions.iter().flat_map(|x| x.bytes()).collect();
    return Instructions(bytes);
}

pub(crate) fn test_constants(checkers: Vec<Box<dyn Fn(&Object, Index)>>, actuals: Vec<Object>) {
    if checkers.len() != actuals.len() {
        panic!(
            "Wrong number of constants expected {:?} got {:?}",
            checkers.len(),
            actuals.len()
        );
    }
    #[cfg(test)]
    println!("Constants: {actuals:?}");
    for (index, value) in checkers.iter().zip(actuals).enumerate() {
        let (checker, actual) = value;
        checker(&actual, index.into())
    }
}

pub(crate) fn test_be_integer(value: i64) -> Box<dyn Fn(&Object, Index)> {
    Box::new(move |object: &Object, i: Index| match object {
        Object::Int(v) => assert_eq!(&value, v, "Expecing {:?} got {:?} at {:?}", value, v, i),
        _ => panic!("Expecting int got {:?} at {:?}", object, i),
    })
}
pub(crate) fn test_be_string<'a>(value: &'a str) -> Box<dyn Fn(&Object, Index) + 'a> {
    Box::new(move |object: &Object, i: Index| match object {
        Object::String(v) => assert_eq!(&value, v, "Expecing {:?} got {:?} at {:?}", value, v, i),
        _ => panic!("Expecting int got {:?} at {:?}", object, i),
    })
}

pub(crate) struct Index(usize);
impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Index(value)
    }
}
