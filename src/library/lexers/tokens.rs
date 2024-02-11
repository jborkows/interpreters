use super::base::{ColumnNumber, LineNumber, SourceCharecter, StateLineContext};

#[derive(Debug, PartialEq, Eq)]
pub struct Token(pub LineNumber, pub ColumnNumber, pub TokenKind);

impl Token {
    pub(crate) fn new(source_charecter: &SourceCharecter, token_kind: TokenKind) -> Self {
        Self(
            source_charecter.line_number,
            source_charecter.column_number,
            token_kind,
        )
    }
    pub fn full(
        line_number: LineNumber,
        column_number: ColumnNumber,
        token_kind: TokenKind,
    ) -> Self {
        Self(line_number, column_number, token_kind)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Illegal(String),
    EOF(),
    EndOfLine(),
    Identifier(String),
    Integer(i32),

    Comma(),
    Semicolon(),
    LeftParen(),
    RightParen(),
    LeftBrace(),
    RightBrace(),

    Function(),
    Let(),

    Assign(),
    Plus(),
    Equality(),
    Inequality(),
    Negation(),
    LessThen(),
    GreaterThen(),
    Slash(),
    Asterisk(),

    True(),
    False(),
    If(),
    Else(),
    Return(),
}

impl StateLineContext {
    pub(crate) fn full_token(&self, token_kind: TokenKind) -> Token {
        Token::full(self.line, self.column, token_kind)
    }
}
