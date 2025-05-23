use super::Parser;
use crate::ast::base::Node;
use crate::ast::expression::Identifier;
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
    assert_eq!(program.statements[0].token_literal(), "let x=5");
    assert_eq!(program.statements[1].token_literal(), "let y=10");
    assert_eq!(program.statements[2].token_literal(), "let foobar=838383");
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
    assert_eq!(program.statements[0].token_literal(), "return 5");
    assert_eq!(program.statements[1].token_literal(), "return 10");
    assert_eq!(program.statements[2].token_literal(), "return 838383");
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
            let identifier = downcast_into!(expression, Identifier);
            assert_eq!(
                identifier.token.kind,
                TokenKind::Identifier("foobar".to_string())
            );
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
            check_if_integer_literal(expression, 5);
        }
        _ => panic!("Expected ExpressionStatement"),
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
                check_if_integer_literal(&operator_expression.right, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

fn check_if_integer_literal(expression: &Box<dyn Expression>, expected_value: u32) {
    let literal = downcast_into!(expression, IntegerLiteral);
    if literal.value() != expected_value {
        panic!(
            "Expected IntegerLiteral with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!("Parser errors: {:?}", parser.errors);
    }
}
