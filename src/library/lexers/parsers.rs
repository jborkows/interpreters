use crate::tokens::TokenKind;

pub(super) fn read_special_character(c: char) -> Option<TokenKind> {
    match c {
        '+' => Some(TokenKind::Plus),
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
        _ => None,
    }
}

pub(super) fn acceptable_separator(c: &char) -> bool {
    return *c == ';' || c.is_whitespace() || *c == '\n' || *c == '\r';
}
