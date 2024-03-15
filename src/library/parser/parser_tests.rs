use crate::parser::parser::ParsingError;
use crate::parser::parser::ParsingErrorKind::*;
use crate::{fake_source::Lines, lexers::read_all};

use crate::lexers::{ColumnNumber, LineNumber, Token, TokenKind::*};

use super::parser::parse;
use super::parser::Statement::*;

#[test]
fn parse_assigment() {
    let input = Lines::new(vec![
        String::from("let x = 5;"),
        String::from("let y = 10;"),
    ]);

    let expected = vec![
        LetStatement {
            token: Token(LineNumber(1), ColumnNumber(1), Let()),
            name: Box::new(IdentifierExpression {
                token: Token(
                    LineNumber(1),
                    ColumnNumber(5),
                    Identifier(String::from("x")),
                ),
            }),
            value: Box::new(LiteralInt {
                token: Token(LineNumber(1), ColumnNumber(9), Integer(5)),
                value: 5,
            }),
        },
        LetStatement {
            token: Token(LineNumber(2), ColumnNumber(1), Let()),
            name: Box::new(IdentifierExpression {
                token: Token(
                    LineNumber(2),
                    ColumnNumber(5),
                    Identifier(String::from("y")),
                ),
            }),
            value: Box::new(LiteralInt {
                token: Token(LineNumber(2), ColumnNumber(9), Integer(10)),
                value: 10,
            }),
        },
    ];
    let statements = parse(read_all(input)).into_iter().collect::<Vec<_>>();
    assert_eq!(&statements.len(), &expected.len());
    expected
        .into_iter()
        .zip(statements.iter())
        .for_each(|(expected, statement)| {
            assert_eq!(statement, &expected);
        });
}

#[test]
fn parse_assigment_with_errors() {
    let input = Lines::new(vec![
        String::from("let x = 5;"),
        String::from("let  = 5;"),
        String::from("let x  5;"),
        String::from("let x = ;"),
    ]);
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
            value: Box::new(LiteralInt {
                token: Token(LineNumber(1), ColumnNumber(8), Integer(5)),
                value: 5,
            }),
        },
        ReturnStatement {
            token: Token(LineNumber(2), ColumnNumber(1), Return()),
            value: Box::new(LiteralInt {
                token: Token(LineNumber(2), ColumnNumber(8), Integer(10)),
                value: 10,
            }),
        },
    ];
    let statements = parse(read_all(input)).into_iter().collect::<Vec<_>>();
    assert_eq!(&statements.len(), &expected.len());
    expected
        .into_iter()
        .zip(statements.iter())
        .for_each(|(expected, statement)| {
            assert_eq!(statement, &expected);
        });
}
