use super::Parser;
use crate::ast::base::Node;
use crate::ast::expression::{BooleanLiteral, Identifier, InfixExpression, InfixOperatorType};
use crate::{
    ast::{
        expression::{Expression, IntegerLiteral, PrefixOperator, PrefixOperatorType},
        statements::Statement,
    },
    tokens::TokenKind,
};

macro_rules! downcast_into {
    ($expr:expr, $target:ty) => {
        $expr.as_any().downcast_ref::<$target>().expect(concat!(
            "Expected ",
            stringify!($target),
            ""
        ))
    };
}

#[test]
fn let_parsing() {
    let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3);
    assert_eq!(program.statements[0].to_string(), "let x=5");
    assert_eq!(program.statements[1].to_string(), "let y=10");
    assert_eq!(program.statements[2].to_string(), "let foobar=838383");
}

#[test]
fn return_parsing() {
    let input = r#"
    return 5;
    return 10;
    return 838383;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3);
    assert_eq!(program.statements[0].to_string(), "return 5");
    assert_eq!(program.statements[1].to_string(), "return 10");
    assert_eq!(program.statements[2].to_string(), "return 838383");
}

#[test]
fn parse_identifier() {
    let input = r#"
    foobar;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            check_if_identifiers_equals(expression, "foobar".to_string());
        }
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_number() {
    let input = r#"
    5;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            check_if_integer_literal_equals(expression, 5);
        }
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_boolean() {
    let inputs = vec![("true;", true), ("false;", false)];
    for (input, value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                check_if_boolean_literal_equals(expression, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn parse_prefix() {
    let inputs = vec![
        ("-5;", PrefixOperatorType::Minus, 5),
        ("!5;", PrefixOperatorType::Bang, 5),
    ];
    for (input, operator, value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let operator_expression = downcast_into!(expression, PrefixOperator);
                assert_eq!(operator_expression.operator, operator);
                check_if_integer_literal_equals(&operator_expression.right, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

fn check_if_identifiers_equals(expression: &Box<dyn Expression>, expected_value: String) {
    let literal = downcast_into!(expression, Identifier);
    if literal.value() != expected_value {
        panic!(
            "Expected Identifier with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
}

fn check_if_boolean_literal_equals(expression: &Box<dyn Expression>, expected_value: bool) {
    let literal = downcast_into!(expression, BooleanLiteral);
    if literal.value != expected_value {
        panic!(
            "Expected Boolean literal with value {}, but got {}",
            expected_value, literal.value
        );
    }
}

fn check_if_integer_literal_equals(expression: &Box<dyn Expression>, expected_value: u32) {
    let literal = downcast_into!(expression, IntegerLiteral);
    if literal.value() != expected_value {
        panic!(
            "Expected IntegerLiteral with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
}

#[test]
fn parse_infix_expression() {
    let inputs = vec![
        ("5 + 4;", InfixOperatorType::Plus, 5, 4),
        ("5 - 4;", InfixOperatorType::Minus, 5, 4),
        ("5 * 4;", InfixOperatorType::Multiply, 5, 4),
        ("5 / 4;", InfixOperatorType::Divide, 5, 4),
        ("5 == 4;", InfixOperatorType::Equal, 5, 4),
        ("5 < 4;", InfixOperatorType::LessThan, 5, 4),
        ("5 > 4;", InfixOperatorType::GreaterThan, 5, 4),
        ("5 != 4;", InfixOperatorType::NotEqual, 5, 4),
    ];
    for (input, operator, left, right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let infix = downcast_into!(expression, InfixExpression);
                assert_eq!(infix.operator, operator);
                check_if_integer_literal_equals(&infix.left, left);
                check_if_integer_literal_equals(&infix.right, right);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn parse_infix_expression_boolean() {
    let inputs = vec![
        ("true == true", InfixOperatorType::Equal, true, true),
        ("true != false", InfixOperatorType::NotEqual, true, false),
        ("false == false", InfixOperatorType::Equal, false, false),
        ("false != true", InfixOperatorType::NotEqual, false, true),
    ];
    for (input, operator, left, right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let infix = downcast_into!(expression, InfixExpression);
                assert_eq!(infix.operator, operator);
                check_if_boolean_literal_equals(&infix.left, left);
                check_if_boolean_literal_equals(&infix.right, right);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn test_precedense_parsing() {
    let inputs = vec![
        ("-a * b", "((-a) * b)"),
        ("!a + b", "((!a) + b)"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c - d / e", "((a + (b * c)) - (d / e))"),
        ("a == b != c", "((a == b) != c)"),
        ("a == b < c", "(a == (b < c))"),
    ];
    for (input, expected) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0].to_string(), expected);
    }
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!("Parser errors: {:?}", parser.errors);
    }
}
