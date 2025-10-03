use std::panic;

use crate::{
    ast::expression::{Expression, InfixOperatorType},
    evaluator::tests::evaluator_tests::eval_input,
    tokens::TokenKind,
};

#[test]
fn should_quote_integer() {
    let result = eval_input("quote(42)");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_integer_literal_equals(&quoted, 42),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_quote_addition() {
    let result = eval_input("quote(4+2)");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match &quoted.as_ref() {
            Expression::Infix {
                token: _,
                left,
                operator: _,
                right,
            } => {
                check_if_integer_literal_equals(left, 4);
                check_if_integer_literal_equals(right, 2);
            }
            _ => panic!(
                "Expected an Infix expression, but got: {}",
                quoted.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_quote_addition_of_ident() {
    let result = eval_input("quote(one+two)");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match quoted.as_ref() {
            Expression::Infix {
                token: _,
                left,
                operator: _,
                right,
            } => {
                check_if_identifiers_equals(left, "one".to_string());
                check_if_identifiers_equals(right, "two".to_string());
            }
            _ => panic!(
                "Expected an Infix expression, but got: {}",
                quoted.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_literal() {
    let result = eval_input("quote(unquote(8))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_integer_literal_equals(quoted, 8),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_funny_literal() {
    let result = eval_input("quote(unquote(--8))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_integer_literal_equals(quoted, 8),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_addition() {
    let result = eval_input("quote(unquote(4+4))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_integer_literal_equals(quoted, 8),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_negative_value() {
    let result = eval_input("quote(unquote(-4))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match quoted.as_ref() {
            Expression::PrefixOperator {
                token: _,
                operator,
                right,
            } => {
                match operator {
                    crate::ast::expression::PrefixOperatorType::Minus => {}
                    _ => panic!("Expected a minus operator, but got: {}", operator),
                }
                check_if_integer_literal_equals(&right, 4);
            }
            _ => panic!(
                "Expected a prefix operator, but got: {}",
                result.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_addition_of_addition() {
    let result = eval_input("quote(4+unquote(4+4))");

    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match quoted.as_ref() {
            Expression::Infix {
                token: _,
                left,
                operator: _,
                right,
            } => {
                check_if_integer_literal_equals(left, 4);
                check_if_integer_literal_equals(right, 8);
            }
            _ => panic!(
                "Expected an Infix expression, but got: {}",
                quoted.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_boolean_literal() {
    let result = eval_input("quote(unquote(true))");

    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_boolean_literal(&quoted, true),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
    let result = eval_input("quote(unquote(false))");

    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_boolean_literal(&quoted, false),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_bolean_expression() {
    let result = eval_input("quote(unquote(true == false))");

    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_boolean_literal(&quoted, false),
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_quote_value() {
    let result = eval_input("quote(unquote(quote(4+4)))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match quoted.as_ref() {
            Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                match operator {
                    InfixOperatorType::Plus => {}
                    _ => panic!("Expected plus got {:?}", operator),
                }
                check_if_integer_literal_equals(&left, 4);
                check_if_integer_literal_equals(&right, 4);
            }
            _ => panic!(
                "Expected a prefix operator, but got: {}",
                result.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

#[test]
fn should_unquote_quote_with_addition_avalue() {
    let result = eval_input("quote(unquote(4+4)+unquote(quote(4+4)))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => match quoted.as_ref() {
            Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                match operator {
                    InfixOperatorType::Plus => {}
                    _ => panic!("Expected plus got {:?}", operator),
                }
                check_if_integer_literal_equals(&left, 8);
                match right.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        match operator {
                            InfixOperatorType::Plus => {}
                            _ => panic!("Expected plus got {:?}", operator),
                        }

                        check_if_integer_literal_equals(&left, 4);
                        check_if_integer_literal_equals(&right, 4);
                    }
                    _ => panic!("expected right to be quoted infix {:?}", right),
                }
            }
            _ => panic!(
                "Expected a prefix operator, but got: {}",
                result.to_string()
            ),
        },
        _ => panic!("Expected a Quote object, but got: {}", result.to_string()),
    }
}

macro_rules! check_expression_value {
    ($expression:expr, $variant:ident, $token_kind:ident, $expected:expr) => {
        match $expression {
            Expression::$variant(inner) => match inner.as_ref().kind {
                TokenKind::$token_kind(ref value) => {
                    if value != &$expected {
                        panic!(
                            concat!(
                                "Expected ",
                                stringify!($token_kind),
                                " with value {}, but got {}"
                            ),
                            $expected, value
                        );
                    }
                }
                _ => panic!(
                    concat!("Expected ", stringify!($token_kind), ", got {:?}"),
                    inner
                ),
            },
            _ => panic!(
                concat!("Expected ", stringify!($variant), ", got {:?}"),
                $expression
            ),
        }
    };
}

fn check_if_integer_literal_equals(expression: &Expression, expected_value: u32) {
    check_expression_value!(expression, IntegerLiteral, Integer, expected_value);
}

fn check_if_boolean_literal(expression: &Expression, expected_value: bool) {
    match expression {
        Expression::BooleanLiteral { token, value: _ } => match token.kind {
            TokenKind::True => {
                if expected_value == false {
                    panic!("Expected false got true")
                }
            }
            TokenKind::False => {
                if expected_value == true {
                    panic!("Expected true got false")
                }
            }

            _ => panic!("Expected boolean got {:?}", token.kind),
        },
        _ => panic!("Expected boolean got {:?}", expression),
    }
}

fn check_if_identifiers_equals(expression: &Expression, expected_value: String) {
    check_expression_value!(expression, Identifier, Identifier, expected_value);
}
