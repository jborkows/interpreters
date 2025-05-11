use crate::lines::{ColumnNumber, LineNumber, SourceCharacter, TextPosition, TokenPosition};

#[derive(Debug)]
pub struct Token {
    pub context: Option<TokenPosition>,
    pub kind: TokenKind,
}

impl Token {
    pub(crate) fn new(context: TokenPosition, token_kind: TokenKind) -> Self {
        Self {
            context: Some(context),
            kind: token_kind,
        }
    }

    pub fn pure(token_kind: TokenKind) -> Self {
        Self {
            context: None,
            kind: token_kind,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Invalid(String),
    Identifier(String),
    StringLiteral(String),
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
    Equal(),
    Inequal(),
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
