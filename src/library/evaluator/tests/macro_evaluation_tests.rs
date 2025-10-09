use std::{cell::RefCell, panic, rc::Rc};

use crate::ast::expression::Expression;
use crate::tokens::TokenKind;

use crate::{
    ast::statements::Program,
    check_expression_value,
    evaluator::{define_macros, expand_macros, tests::evaluator_tests::check_parser_errors},
    parser::Parser,
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

fn prepare_for_evaluation_modify_program(input: &str) -> Program {
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    let env = Rc::new(RefCell::new(crate::object::Environment::new()));
    let after_macro_population_to_env = define_macros(program, env.clone());
    let expended_macro = expand_macros(after_macro_population_to_env, env.clone());
    return expended_macro;
}

#[test]
fn expand_macro_for_simple_infix() {
    let program = prepare_for_evaluation_modify_program(
        r#"
        let infix = macro() { quote(1 + 2); };
        infix()
    "#,
    );
    assert_eq!(program.statements.len(), 1);
    let statement = program.statements.get(0).unwrap();
    match statement {
        crate::ast::statements::Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            crate::ast::expression::Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                check_expression_value!(left.as_ref(), IntegerLiteral, Integer, 1);
                check_expression_value!(right.as_ref(), IntegerLiteral, Integer, 2);
                match operator {
                    crate::ast::expression::InfixOperatorType::Plus => {}
                    _ => panic!("Expected plus got {:?}", operator),
                }
            }

            _ => panic!("Expected infix got {:?}", expression),
        },
        _ => panic!("Expected expression got {:?}", statement),
    }
}

#[test]
fn expand_macro_for_complex_infixc() {
    let program = prepare_for_evaluation_modify_program(
        r#"
        let complex = macro(a,b) { quote(unquote(b) - unquote(a)); };
        complex(2+2,5-2)
    "#,
    );
    assert_eq!(program.statements.len(), 1);
    let statement = program.statements.get(0).unwrap();
    match statement {
        crate::ast::statements::Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            crate::ast::expression::Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                match operator {
                    crate::ast::expression::InfixOperatorType::Minus => {}
                    _ => panic!("Expected minus got {:?}", operator),
                };
                match left.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        check_expression_value!(left.as_ref(), IntegerLiteral, Integer, 5);
                        check_expression_value!(right.as_ref(), IntegerLiteral, Integer, 2);

                        match operator {
                            crate::ast::expression::InfixOperatorType::Minus => {}
                            _ => panic!("Expected minus got {:?}", operator),
                        };
                    }
                    _ => panic!("Expected infix {:?}", left),
                }

                match right.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        check_expression_value!(left.as_ref(), IntegerLiteral, Integer, 2);
                        check_expression_value!(right.as_ref(), IntegerLiteral, Integer, 2);

                        match operator {
                            crate::ast::expression::InfixOperatorType::Plus => {}
                            _ => panic!("Expected plus got {:?}", operator),
                        };
                    }
                    _ => panic!("Expected infix {:?}", left),
                }
            }

            _ => panic!("Expected infix got {:?}", expression),
        },
        _ => panic!("Expected expression got {:?}", statement),
    }
}
