use std::rc::Rc;

use super::lexer::Lexer;
use crate::lines::{ColumnNumber, LineNumber, TextPosition, TokenPosition};
use crate::tokens::{Token, TokenKind};

#[test]
fn next_sign() {
    let input = vec!["=+(){},;*<;>/;"];

    let expected = vec![
        (single(1, 1), TokenKind::Assign),
        (single(1, 2), TokenKind::Plus),
        (single(1, 3), TokenKind::LeftParen),
        (single(1, 4), TokenKind::RightParen),
        (single(1, 5), TokenKind::LeftBrace),
        (single(1, 6), TokenKind::RightBrace),
        (single(1, 7), TokenKind::Comma),
        (single(1, 8), TokenKind::Semicolon),
        (single(1, 9), TokenKind::Asterisk),
        (single(1, 10), TokenKind::LessThen),
        (single(1, 11), TokenKind::Semicolon),
        (single(1, 12), TokenKind::GreaterThen),
        (single(1, 13), TokenKind::Slash),
        (single(1, 14), TokenKind::Semicolon),
    ];

    perform_test(input, expected);
}

#[test]
fn euqality_negation() {
    let input = vec!["==!=!;"];
    let expected = vec![
        (position(1, 1, 1, 2), TokenKind::Equal),
        (position(1, 3, 1, 4), TokenKind::Inequal),
        (single(1, 5), TokenKind::Negation),
        (single(1, 6), TokenKind::Semicolon),
    ];

    perform_test(input, expected);
}

#[test]
fn true_false() {
    let input = vec!["true false trues falses"];
    let expected = vec![
        (position(1, 1, 1, 4), TokenKind::True),
        (position(1, 6, 1, 10), TokenKind::False),
        (
            position(1, 12, 1, 16),
            TokenKind::Identifier(String::from("trues")),
        ),
        (
            position(1, 18, 1, 23),
            TokenKind::Identifier(String::from("falses")),
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn numbers_and_false_numbers() {
    let input = vec!["0 123 123a -1"];
    let expected = vec![
        (position(1, 1, 1, 1), TokenKind::Integer(0)),
        (position(1, 3, 1, 5), TokenKind::Integer(123)),
        (
            position(1, 7, 1, 10),
            TokenKind::Invalid(String::from("Unexpected character 'a' in number")),
        ),
        (single(1, 12), TokenKind::Minus),
        (position(1, 13, 1, 13), TokenKind::Integer(1)),
    ];

    perform_test(input, expected);
}

#[test]
fn if_else_return() {
    let input = vec!["if ifs else elses"];
    let expected = vec![
        (position(1, 1, 1, 2), TokenKind::If),
        (
            position(1, 4, 1, 6),
            TokenKind::Identifier(String::from("ifs")),
        ),
        (position(1, 8, 1, 11), TokenKind::Else),
        (
            position(1, 13, 1, 17),
            TokenKind::Identifier(String::from("elses")),
        ),
    ];

    perform_test(input, expected);
    let input = vec!["return returns"];
    let expected = vec![
        (position(1, 1, 1, 6), TokenKind::Return),
        (
            position(1, 8, 1, 14),
            TokenKind::Identifier(String::from("returns")),
        ),
    ];
    perform_test(input, expected);
}

#[test]
fn multiline_string() {
    let input = vec!["\" first", "line\""];
    let expected = vec![(
        position(1, 1, 2, 5),
        TokenKind::StringLiteral(String::from(" first\nline")),
    )];
    perform_test(input, expected);
}

#[test]
fn invalid_multiline_string() {
    let input = vec!["\" first", "line"];
    let expected = vec![(
        position(1, 1, 2, 4),
        TokenKind::Invalid(String::from("Unclosed string literal")),
    )];
    perform_test(input, expected);
}

#[test]
fn brackets() {
    let input = vec!["[1, 2]"];
    let expected = vec![
        (single(1, 1), TokenKind::LeftBracket),
        (single(1, 2), TokenKind::Integer(1)),
        (single(1, 3), TokenKind::Comma),
        (single(1, 5), TokenKind::Integer(2)),
        (single(1, 6), TokenKind::RightBracket),
    ];
    perform_test(input, expected);
}

#[test]
fn more_complex_text() {
    let input = vec![
        "let five = 5;",
        "let ten = 10;",
        "",
        "let add = fn(x, y) {",
        "  x + y;",
        "};",
        "",
        "let result = add(five, ten);",
    ];
    let expected = vec![
        (position(1, 1, 1, 3), TokenKind::Let),
        (
            position(1, 5, 1, 8),
            TokenKind::Identifier(String::from("five")),
        ),
        (position(1, 10, 1, 10), TokenKind::Assign),
        (position(1, 12, 1, 12), TokenKind::Integer(5)),
        (position(1, 13, 1, 13), TokenKind::Semicolon),
        (position(2, 1, 2, 3), TokenKind::Let),
        (
            position(2, 5, 2, 7),
            TokenKind::Identifier(String::from("ten")),
        ),
        (position(2, 9, 2, 9), TokenKind::Assign),
        (position(2, 11, 2, 12), TokenKind::Integer(10)),
        (position(2, 13, 2, 13), TokenKind::Semicolon),
        //"let add = fn(x, y) {",
        (position(4, 1, 4, 3), TokenKind::Let),
        (
            position(4, 5, 4, 7),
            TokenKind::Identifier(String::from("add")),
        ),
        (position(4, 9, 4, 9), TokenKind::Assign),
        (position(4, 11, 4, 12), TokenKind::Function),
        (single(4, 13), TokenKind::LeftParen),
        (single(4, 14), TokenKind::Identifier(String::from("x"))),
        (single(4, 15), TokenKind::Comma),
        (single(4, 17), TokenKind::Identifier(String::from("y"))),
        (single(4, 18), TokenKind::RightParen),
        (single(4, 20), TokenKind::LeftBrace),
        //"  x + y;"
        (single(5, 3), TokenKind::Identifier(String::from("x"))),
        (single(5, 5), TokenKind::Plus),
        (single(5, 7), TokenKind::Identifier(String::from("y"))),
        (single(5, 8), TokenKind::Semicolon),
        //"};",
        (single(6, 1), TokenKind::RightBrace),
        (single(6, 2), TokenKind::Semicolon),
        // "let result = add(five, ten);",
        (position(8, 1, 8, 3), TokenKind::Let),
        (
            position(8, 5, 8, 10),
            TokenKind::Identifier(String::from("result")),
        ),
        (single(8, 12), TokenKind::Assign),
        (
            position(8, 14, 8, 16),
            TokenKind::Identifier(String::from("add")),
        ),
        (single(8, 17), TokenKind::LeftParen),
        (
            position(8, 18, 8, 21),
            TokenKind::Identifier(String::from("five")),
        ),
        (single(8, 22), TokenKind::Comma),
        (
            position(8, 24, 8, 26),
            TokenKind::Identifier(String::from("ten")),
        ),
        (single(8, 27), TokenKind::RightParen),
        (single(8, 28), TokenKind::Semicolon),
    ];

    perform_test(input, expected);
}

fn perform_test(input: Vec<&str>, expected: Vec<(TokenPosition, TokenKind)>) {
    println!("Expected:");
    for exp in expected.iter() {
        println!("{:?}", exp);
    }

    let mut lexer = Lexer::new();
    for line in input.iter() {
        lexer.process(line);
    }
    let tokens: Vec<Rc<Token>> = lexer.into_iter().collect();
    assert_eq!(
        tokens.len(),
        expected.len(),
        "Token count mismatch got {} expected {} - existing {}",
        tokens.len(),
        expected.len(),
        tokens
            .iter()
            .map(|token| format!("{:?}", token.kind))
            .collect::<Vec<_>>()
            .join(", ")
    );

    tokens
        .into_iter()
        .zip(expected.iter())
        .for_each(|(token, expected)| {
            assert_eq!(token.kind, expected.1);
            let position = token.context.unwrap();
            assert_eq!(
                position, expected.0,
                "Position mismatch for token {:?}",
                token.kind
            );
        });
}

pub fn position(
    start_line: u16,
    start_column: u16,
    end_line: u16,
    end_column: u16,
) -> TokenPosition {
    TokenPosition {
        start: TextPosition {
            line_number: LineNumber(start_line),
            column_number: ColumnNumber(start_column),
        },
        end: TextPosition {
            line_number: LineNumber(end_line),
            column_number: ColumnNumber(end_column),
        },
    }
}
pub fn single(line_number: u16, column_number: u16) -> TokenPosition {
    TokenPosition {
        start: TextPosition {
            line_number: LineNumber(line_number),
            column_number: ColumnNumber(column_number),
        },
        end: TextPosition {
            line_number: LineNumber(line_number),
            column_number: ColumnNumber(column_number),
        },
    }
}
