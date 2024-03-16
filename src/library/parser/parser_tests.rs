use super::types::ParsingError;
use super::types::ParsingErrorKind::*;
use crate::parser::types::Expression;
use crate::{fake_source::Lines, lexers::read_all};

use crate::lexers::{ColumnNumber, LineNumber, Token, TokenKind::*};

use super::parser::parse;
use super::types::Statement::*;

#[test]
fn parse_assigment() {
    let input = Lines::m(vec!["let x = 5;", "let y = 10;"]);

    let expected = vec![
        LetStatement {
            token: Token(LineNumber(1), ColumnNumber(1), Let()),
            name: Box::new(ExpressionStatement {
                token: Token(
                    LineNumber(1),
                    ColumnNumber(5),
                    Identifier(String::from("x")),
                ),
                expression: Box::new(Expression::IdentifierExpression {
                    name: String::from("x"),
                }),
            }),
            value: Box::new(ExpressionStatement {
                token: Token(LineNumber(1), ColumnNumber(9), Integer(5)),
                expression: Box::new(Expression::LiteralInt { value: 5 }),
            }),
        },
        LetStatement {
            token: Token(LineNumber(2), ColumnNumber(1), Let()),
            name: Box::new(ExpressionStatement {
                token: Token(
                    LineNumber(2),
                    ColumnNumber(5),
                    Identifier(String::from("y")),
                ),
                expression: Box::new(Expression::IdentifierExpression {
                    name: String::from("y"),
                }),
            }),
            value: Box::new(ExpressionStatement {
                token: Token(LineNumber(2), ColumnNumber(9), Integer(10)),
                expression: Box::new(Expression::LiteralInt { value: 10 }),
            }),
        },
    ];

    let program = pase_input(input);
    let statements = program.into_iter().collect::<Vec<_>>();
    assert_eq!(&statements.len(), &expected.len());
    expected
        .into_iter()
        .zip(statements.iter())
        .for_each(|(expected, statement)| {
            assert_eq!(statement, &expected);
        });
}

fn pase_input(input: Lines) -> super::Program {
    let program = parse(read_all(input));
    println!("\nProgram: \n{}\nEnd\n", program.to_string());
    program
}

#[test]
fn parse_assigment_with_errors() {
    let input = Lines::m(vec!["let x = 5;", "let  = 5;", "let x  5;", "let x = ;"]);
    let program = parse(read_all(input));
    let expected_errors = vec![
        ParsingError {
            message: ExpectedIdentifier,
            line: LineNumber(2),
            column: ColumnNumber(6),
        },
        ParsingError {
            message: ExpectedAssign(Some(Integer(5))),
            line: LineNumber(3),
            column: ColumnNumber(8),
        },
        ParsingError {
            message: ExpectedInteger,
            line: LineNumber(4),
            column: ColumnNumber(9),
        },
    ];

    expected_errors.iter().for_each(|expected| {
        assert!(program.errors().contains(expected));
    });
}
#[test]
fn parse_return() {
    let input = Lines::m(vec!["return 5;", "return 10;"]);

    let expected = vec![
        ReturnStatement {
            token: Token(LineNumber(1), ColumnNumber(1), Return()),
            value: Box::new(ExpressionStatement {
                token: Token(LineNumber(1), ColumnNumber(8), Integer(5)),
                expression: Box::new(Expression::LiteralInt { value: 5 }),
            }),
        },
        ReturnStatement {
            token: Token(LineNumber(2), ColumnNumber(1), Return()),
            value: Box::new(ExpressionStatement {
                token: Token(LineNumber(2), ColumnNumber(8), Integer(10)),
                expression: Box::new(Expression::LiteralInt { value: 10 }),
            }),
        },
    ];
    let program = pase_input(input);
    let statements = program.into_iter().collect::<Vec<_>>();
    assert_eq!(&statements.len(), &expected.len());
    expected
        .into_iter()
        .zip(statements.iter())
        .for_each(|(expected, statement)| {
            assert_eq!(statement, &expected);
        });
}
