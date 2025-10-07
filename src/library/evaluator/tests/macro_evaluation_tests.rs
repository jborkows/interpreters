use std::{cell::RefCell, panic, rc::Rc};

use crate::{
    evaluator::{
        define_macros, evaluate,
        tests::{self, evaluator_tests::check_parser_errors},
    },
    join_collection,
    object::Object,
    parser::Parser,
    print_bash_error,
};
fn prepare_for_evaluation(input: &str) -> Rc<RefCell<crate::object::Environment>> {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    let env = Rc::new(RefCell::new(crate::object::Environment::new()));
    define_macros(program, env.clone());
    env.clone()
    //(evaluate(&modified, env.clone()), env.clone()) <- we want to prove that macro is populated
    //into environment before evaluation
}

#[test]
fn defining_macros() {
    let text = r#"
       let number = 1
       let function = fn(a,b){a+b}
       let mymacro = macro(a,b){a+b}
    "#;

    let env = prepare_for_evaluation(text);
    if let Some(_) = env.borrow().get("number") {
        panic!("Number should not be in evironment")
    }

    if let Some(_) = env.borrow().get("function") {
        panic!("function should not be in evironment")
    }

    if let Some(value) = env.borrow().get("mymacro") {
        match value.as_ref() {
            crate::object::Object::Macro {
                parameters,
                body,
                env: _,
            } => {
                assert_eq!(parameters.len(), 2);
                assert_eq!(parameters[0].to_string(), "a");
                assert_eq!(parameters[1].to_string(), "b");
                assert_eq!(body.to_string(), "(a + b)");
            }
            _ => panic!("Expected a Function object, but got: {}", value.to_string()),
        }
        return;
    }

    panic!("Should not get here")
}
