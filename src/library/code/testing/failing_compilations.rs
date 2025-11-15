use core::panic;

use crate::code::{CompilationError, compile, testing::test_compiler::parse_program};

macro_rules! tests {

    ($($name:ident: ($program_text:expr,$checkers:expr),)*) => {
        $(
            #[test]
            fn $name() {
                failing_compilation($program_text, $checkers);
            }
        )*
    };

}

fn failing_compilation(input: &str, checkers: Vec<Box<dyn Fn(&Vec<CompilationError>)>>) {
    let program = parse_program(input);
    let compiled = compile(program);
    let e = match compiled {
        Ok(_) => panic!("Should not compile"),
        Err(e) => e,
    };
    for checker in checkers {
        checker(&e)
    }
}

tests! {
  function_literal: (
        "fn(a,b){}(1)",
        vec![
            should_fail_with_arg(ArgumentMismatch { expected_arguments_size: 2, given_arguments_size: 1 })
        ]),
    /*FIXME: to make it work I would need to know type of expression same goes for "named"
    functions
 function_returning_function:(
         "fn(){ fn(a,b){} }()(1)",
         vec![
             should_fail_with_arg(ArgumentMismatch { expected_arguments_size: 2, given_arguments_size: 1 })
         ]),
             */
}

struct ArgumentMismatch {
    expected_arguments_size: usize,
    given_arguments_size: usize,
}
fn should_fail_with_arg(mismatch: ArgumentMismatch) -> Box<dyn Fn(&Vec<CompilationError>)> {
    Box::new(move |errors: &Vec<CompilationError>| {
        for error in errors {
            match error {
                CompilationError::WrongNumberOfArguments {
                    token: _,
                    expected,
                    provided,
                } => {
                    if expected == &mismatch.expected_arguments_size
                        && &mismatch.given_arguments_size == provided
                    {
                        return;
                    }
                }
                _ => {}
            }
        }
        panic!("No error about argument size errors, found {errors:?}")
    })
}
