use super::lexer::Lexer;
use crate::lines::{ColumnNumber, LineNumber, TokenPosition};
use crate::tokens::{Token, TokenKind};

#[test]
fn next_sign() {
    let input = vec!["=+(){},;*<;>/;"];

    let expected = vec![
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(1)),
            TokenKind::Assign,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(2)),
            TokenKind::Plus,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(3)),
            TokenKind::LeftParen,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(4)),
            TokenKind::RightParen,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(5)),
            TokenKind::LeftBrace,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(6)),
            TokenKind::RightBrace,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(7)),
            TokenKind::Comma,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(8)),
            TokenKind::Semicolon,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(9)),
            TokenKind::Asterisk,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(10)),
            TokenKind::LessThen,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(11)),
            TokenKind::Semicolon,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(12)),
            TokenKind::GreaterThen,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(13)),
            TokenKind::Slash,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(14)),
            TokenKind::Semicolon,
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn euqality_negation() {
    let input = vec!["==!=!;"];
    let expected = vec![
        (TokenPosition::from_range(1, 1, 1, 2), TokenKind::Equal),
        (TokenPosition::from_range(1, 3, 1, 4), TokenKind::Inequal),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(5)),
            TokenKind::Negation,
        ),
        (
            TokenPosition::single_character(LineNumber(1), ColumnNumber(6)),
            TokenKind::Semicolon,
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn true_false() {
    let input = vec!["true false trues falses"];
    let expected = vec![
        (TokenPosition::from_range(1, 1, 1, 4), TokenKind::True),
        (TokenPosition::from_range(1, 6, 1, 10), TokenKind::False),
        (
            TokenPosition::from_range(1, 12, 1, 16),
            TokenKind::Identifier(String::from("trues")),
        ),
        (
            TokenPosition::from_range(1, 18, 1, 23),
            TokenKind::Identifier(String::from("falses")),
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn numbers_and_false_numbers() {
    let input = vec!["0 123 123a -1"];
    let expected = vec![
        (TokenPosition::from_range(1, 1, 1, 1), TokenKind::Integer(0)),
        (
            TokenPosition::from_range(1, 3, 1, 5),
            TokenKind::Integer(123),
        ),
        (
            TokenPosition::from_range(1, 7, 1, 10),
            TokenKind::Invalid(String::from("Unexpected character 'a' in number")),
        ),
        (
            TokenPosition::from_range(1, 12, 1, 13),
            TokenKind::Integer(-1),
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn if_else_return() {
    let input = vec!["if ifs else elses"];
    let expected = vec![
        (TokenPosition::from_range(1, 1, 1, 2), TokenKind::If),
        (
            TokenPosition::from_range(1, 4, 1, 6),
            TokenKind::Identifier(String::from("ifs")),
        ),
        (TokenPosition::from_range(1, 8, 1, 11), TokenKind::Else),
        (
            TokenPosition::from_range(1, 13, 1, 17),
            TokenKind::Identifier(String::from("elses")),
        ),
    ];

    perform_test(input, expected);
    let input = vec!["return returns"];
    let expected = vec![
        (TokenPosition::from_range(1, 1, 1, 6), TokenKind::Return),
        (
            TokenPosition::from_range(1, 8, 1, 14),
            TokenKind::Identifier(String::from("returns")),
        ),
    ];
    perform_test(input, expected);
}

#[test]
fn multiline_string() {
    let input = vec!["\" first", "line\""];
    let expected = vec![(
        TokenPosition::from_range(1, 1, 2, 5),
        TokenKind::StringLiteral(String::from(" first\nline")),
    )];
    perform_test(input, expected);
}

#[test]
fn invalid_multiline_string() {
    let input = vec!["\" first", "line"];
    let expected = vec![(
        TokenPosition::from_range(1, 1, 2, 4),
        TokenKind::Invalid(String::from("Unclosed string literal")),
    )];
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
        (TokenPosition::from_range(1, 1, 1, 3), TokenKind::Let),
        (
            TokenPosition::from_range(1, 5, 1, 8),
            TokenKind::Identifier(String::from("five")),
        ),
        (TokenPosition::from_range(1, 10, 1, 10), TokenKind::Assign),
        (
            TokenPosition::from_range(1, 12, 1, 12),
            TokenKind::Integer(5),
        ),
        (
            TokenPosition::from_range(1, 13, 1, 13),
            TokenKind::Semicolon,
        ),
        (TokenPosition::from_range(2, 1, 2, 3), TokenKind::Let),
        (
            TokenPosition::from_range(2, 5, 2, 7),
            TokenKind::Identifier(String::from("ten")),
        ),
        (TokenPosition::from_range(2, 9, 2, 9), TokenKind::Assign),
        (
            TokenPosition::from_range(2, 11, 2, 12),
            TokenKind::Integer(10),
        ),
        (
            TokenPosition::from_range(2, 13, 2, 13),
            TokenKind::Semicolon,
        ),
        //"let add = fn(x, y) {",
        (TokenPosition::from_range(4, 1, 4, 3), TokenKind::Let),
        (
            TokenPosition::from_range(4, 5, 4, 7),
            TokenKind::Identifier(String::from("add")),
        ),
        (TokenPosition::from_range(4, 9, 4, 9), TokenKind::Assign),
        (TokenPosition::from_range(4, 11, 4, 12), TokenKind::Function),
        (TokenPosition::single(4, 13), TokenKind::LeftParen),
        (
            TokenPosition::single(4, 14),
            TokenKind::Identifier(String::from("x")),
        ),
        (TokenPosition::single(4, 15), TokenKind::Comma),
        (
            TokenPosition::single(4, 17),
            TokenKind::Identifier(String::from("y")),
        ),
        (TokenPosition::single(4, 18), TokenKind::RightParen),
        (TokenPosition::single(4, 20), TokenKind::LeftBrace),
        //"  x + y;"
        (
            TokenPosition::single(5, 3),
            TokenKind::Identifier(String::from("x")),
        ),
        (TokenPosition::single(5, 5), TokenKind::Plus),
        (
            TokenPosition::single(5, 7),
            TokenKind::Identifier(String::from("y")),
        ),
        (TokenPosition::single(5, 8), TokenKind::Semicolon),
        //"};",
        (TokenPosition::single(6, 1), TokenKind::RightBrace),
        (TokenPosition::single(6, 2), TokenKind::Semicolon),
        // "let result = add(five, ten);",
        (TokenPosition::from_range(8, 1, 8, 3), TokenKind::Let),
        (
            TokenPosition::from_range(8, 5, 8, 10),
            TokenKind::Identifier(String::from("result")),
        ),
        (TokenPosition::single(8, 12), TokenKind::Assign),
        (
            TokenPosition::from_range(8, 14, 8, 16),
            TokenKind::Identifier(String::from("add")),
        ),
        (TokenPosition::single(8, 17), TokenKind::LeftParen),
        (
            TokenPosition::from_range(8, 18, 8, 21),
            TokenKind::Identifier(String::from("five")),
        ),
        (TokenPosition::single(8, 22), TokenKind::Comma),
        (
            TokenPosition::from_range(8, 24, 8, 26),
            TokenKind::Identifier(String::from("ten")),
        ),
        (TokenPosition::single(8, 27), TokenKind::RightParen),
        (TokenPosition::single(8, 28), TokenKind::Semicolon),
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
    let tokens: Vec<Token> = lexer.into_iter().collect();
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
