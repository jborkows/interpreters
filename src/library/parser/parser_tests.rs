use super::Parser;
use crate::{
    ast::{Node, expression::ExpressionKind, statements::Statement},
    tokens::TokenKind,
};

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
        Statement::ExpressionStatement { token, expression } => {
            assert_eq!(token.kind, TokenKind::Identifier("foobar".to_string()));
            assert_eq!(expression.expression_kind(), ExpressionKind::Identifier);
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
        Statement::ExpressionStatement { token, expression } => {
            assert_eq!(token.kind, TokenKind::Integer(5));
            assert_eq!(expression.expression_kind(), ExpressionKind::IntegerLiteral);
        }
        _ => panic!("Expected ExpressionStatement"),
    }
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!("Parser errors: {:?}", parser.errors);
    }
}
