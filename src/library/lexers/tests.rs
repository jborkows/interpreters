use std::cell::RefCell;

use crate::lexers::base::*;
use crate::lexers::tokens::{Token, TokenKind};

use super::tokens::NextToken;

struct StringLexable {
    text: String,
    called: RefCell<bool>,
}

impl StringLexable {
    fn new(text: String) -> Self {
        Self {
            text,
            called: RefCell::new(false),
        }
    }
}
impl Lexable for StringLexable {
    fn next_line(&self) -> Option<(LineNumber, String)> {
        if self.called.borrow().clone() {
            return None;
        } else {
            self.called.replace_with(|_any| true);
            return Some((LineNumber(1), self.text.clone()));
        }
    }
}

struct StringsLexable {
    texts: Vec<String>,
    current_line: RefCell<usize>,
}
impl StringsLexable {
    fn new(texts: Vec<String>) -> Self {
        Self {
            texts,
            current_line: RefCell::new(0),
        }
    }
}
impl Lexable for StringsLexable {
    fn next_line(&self) -> Option<(LineNumber, String)> {
        let current_line = self.current_line.borrow().clone();
        if current_line >= self.texts.len() {
            return None;
        } else {
            let current_line_value = self.current_line.borrow().clone();
            self.current_line
                .replace_with(|value| value.wrapping_add(1));
            return Some((
                LineNumber(current_line_value as u16 + 1),
                self.texts[current_line_value].clone(),
            ));
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(4, 4);
}

// #[test]
fn next_token_test() {
    let input = StringLexable::new(String::from("=+(){},;"));
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::Assign()),
        (LineNumber(1), ColumnNumber(2), TokenKind::Plus()),
        (LineNumber(1), ColumnNumber(3), TokenKind::LeftParen()),
        (LineNumber(1), ColumnNumber(4), TokenKind::RightParen()),
        (LineNumber(1), ColumnNumber(5), TokenKind::LeftBrace()),
        (LineNumber(1), ColumnNumber(6), TokenKind::RightBrace()),
        (LineNumber(1), ColumnNumber(7), TokenKind::Comma()),
        (LineNumber(1), ColumnNumber(8), TokenKind::Semicolon()),
        (LineNumber(2), ColumnNumber(1), TokenKind::EOF()),
    ];
    let next_token = NextToken::new(&input);
    for (i, tok) in next_token.enumerate() {
        assert_eq!(
            tok,
            Token(expected[i].0, expected[i].1, expected[i].2.clone())
        );
    }
}

#[test]
fn more_complex_text() {
    let input = StringsLexable::new(vec![
        String::from("let five = 5;"),
        String::from("let ten = 10;"),
        String::from(""),
        String::from("let add = fn(x, y) {"),
        String::from("  x + y;"),
        String::from("};"),
        String::from(""),
        String::from("let result = add(five, ten);"),
    ]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(1),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("five")),
        ),
        (LineNumber(1), ColumnNumber(10), TokenKind::Assign()),
        (LineNumber(1), ColumnNumber(12), TokenKind::Integer(5)),
        (LineNumber(1), ColumnNumber(13), TokenKind::Semicolon()),
        (LineNumber(2), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(2),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("ten")),
        ),
        (LineNumber(2), ColumnNumber(9), TokenKind::Assign()),
        (LineNumber(2), ColumnNumber(11), TokenKind::Integer(10)),
        (LineNumber(2), ColumnNumber(13), TokenKind::Semicolon()),
        (LineNumber(4), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(4),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("add")),
        ),
        (LineNumber(4), ColumnNumber(9), TokenKind::Assign()),
        (LineNumber(4), ColumnNumber(11), TokenKind::Function()),
        (LineNumber(4), ColumnNumber(14), TokenKind::LeftParen()),
        (
            LineNumber(4),
            ColumnNumber(15),
            TokenKind::Identifier(String::from("x")),
        ),
        (LineNumber(4), ColumnNumber(16), TokenKind::Comma()),
        (
            LineNumber(4),
            ColumnNumber(18),
            TokenKind::Identifier(String::from("y")),
        ),
        (LineNumber(4), ColumnNumber(19), TokenKind::RightParen()),
        (LineNumber(4), ColumnNumber(21), TokenKind::LeftBrace()),
        (
            LineNumber(5),
            ColumnNumber(3),
            TokenKind::Identifier(String::from("x")),
        ),
        (LineNumber(5), ColumnNumber(5), TokenKind::Plus()),
        (
            LineNumber(5),
            ColumnNumber(7),
            TokenKind::Identifier(String::from("y")),
        ),
        (LineNumber(5), ColumnNumber(8), TokenKind::Semicolon()),
        (LineNumber(6), ColumnNumber(2), TokenKind::RightBrace()),
        (LineNumber(6), ColumnNumber(3), TokenKind::Semicolon()),
        (LineNumber(8), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(8),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("result")),
        ),
        (LineNumber(8), ColumnNumber(12), TokenKind::Assign()),
        (
            LineNumber(8),
            ColumnNumber(14),
            TokenKind::Identifier(String::from("add")),
        ),
        (LineNumber(8), ColumnNumber(17), TokenKind::LeftParen()),
        (
            LineNumber(8),
            ColumnNumber(18),
            TokenKind::Identifier(String::from("five")),
        ),
        (LineNumber(8), ColumnNumber(22), TokenKind::Comma()),
        (
            LineNumber(8),
            ColumnNumber(24),
            TokenKind::Identifier(String::from("ten")),
        ),
        (LineNumber(8), ColumnNumber(27), TokenKind::RightParen()),
        (LineNumber(8), ColumnNumber(28), TokenKind::Semicolon()),
        (LineNumber(9), ColumnNumber(1), TokenKind::EOF()),
    ];

    let next_token = NextToken::new(&input);
    for (i, tok) in next_token.enumerate() {
        assert_eq!(
            tok,
            Token(expected[i].0, expected[i].1, expected[i].2.clone())
        );
    }
}
