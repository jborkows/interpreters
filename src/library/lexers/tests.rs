use std::cell::RefCell;

use crate::lexers::base::{ColumnNumber, Lexable, LineNumber};
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

#[test]
fn it_works() {
    assert_eq!(4, 4);
}

#[test]
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
        (LineNumber(1), ColumnNumber(9), TokenKind::EOF()),
    ];
    let next_token = NextToken::new(&input);
    for (i, tok) in next_token.enumerate() {
        assert_eq!(
            tok,
            Token(expected[i].0, expected[i].1, expected[i].2.clone())
        );
    }
}
