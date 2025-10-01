use crate::{
    ast::expression::Expression, evaluator::tests::evaluator_tests::eval_input, tokens::TokenKind,
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
fn should_unquote_addition() {
    let result = eval_input("quote(unquote(4+4))");
    match result.as_ref() {
        crate::object::Object::Quote(quoted) => check_if_integer_literal_equals(quoted, 8),
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

fn check_if_identifiers_equals(expression: &Expression, expected_value: String) {
    check_expression_value!(expression, Identifier, Identifier, expected_value);
}
