use crate::{
    lines::TokenPosition,
    tokens::{Token, TokenKind},
};

use super::{dispatch::dispatch, parsing_states::LexerState};

pub(super) fn read_special_character(c: char) -> Option<TokenKind> {
    match c {
        '+' => Some(TokenKind::Plus),
        '-' => Some(TokenKind::Minus),
        '(' => Some(TokenKind::LeftParen),
        ')' => Some(TokenKind::RightParen),
        '{' => Some(TokenKind::LeftBrace),
        '}' => Some(TokenKind::RightBrace),
        ',' => Some(TokenKind::Comma),
        ';' => Some(TokenKind::Semicolon),
        '*' => Some(TokenKind::Asterisk),
        '<' => Some(TokenKind::LessThen),
        '>' => Some(TokenKind::GreaterThen),
        '/' => Some(TokenKind::Slash),
        ']' => Some(TokenKind::RightBracket),
        '[' => Some(TokenKind::LeftBracket),
        ':' => Some(TokenKind::Collon),
        _ => None,
    }
}

pub(super) fn acceptable_separator(c: &char) -> bool {
    *c == ';' || c.is_whitespace() || *c == '\n' || *c == '\r' || *c == ','
}

pub(super) fn delegate_to_next<P>(
    character: char,
    column_number: u16,
    line_number: u16,
    token_kind: TokenKind,
    token_position: P,
) -> (LexerState, Vec<Token>)
where
    P: Fn() -> TokenPosition,
{
    delegate_to_next_position(
        character,
        column_number,
        line_number,
        token_kind,
        token_position(),
    )
}

pub(super) fn delegate_to_next_position(
    character: char,
    column_number: u16,
    line_number: u16,
    token_kind: TokenKind,
    token_position: TokenPosition,
) -> (LexerState, Vec<Token>) {
    let mut tokens = vec![Token::new(token_position, token_kind)];
    let result = dispatch(line_number, column_number, character, &LexerState::Idle);
    tokens.extend(result.1);
    (result.0, tokens)
}
