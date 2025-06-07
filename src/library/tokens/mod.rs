use std::fmt::Display;

use crate::lines::TokenPosition;

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

    pub fn short(&self) -> String {
        self.kind.literal()
    }

    pub fn position(&self) -> (usize, usize) {
        if let Some(context) = &self.context {
            (
                context.start.line_number.0.into(),
                context.start.column_number.0.into(),
            )
        } else {
            (0, 0)
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(context) = &self.context {
            write!(f, "{}: {}", context, self.kind.literal())
        } else {
            write!(f, "{}", self.kind.literal())
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Invalid(String),
    Identifier(String),
    StringLiteral(String),
    Integer(u32),
    Minus,

    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Function,
    Let,

    Assign,
    Plus,
    Equal,
    Inequal,
    Negation,
    LessThen,
    GreaterThen,
    Slash,
    Asterisk,

    True,
    False,
    If,
    Else,
    Return,
    Collon,
}

impl TokenKind {
    pub(crate) fn literal(&self) -> String {
        match self {
            TokenKind::Invalid(s) => format!("Invalid({})", s),
            TokenKind::Identifier(s) => s.to_string(),
            TokenKind::StringLiteral(s) => s.to_string(),
            TokenKind::Integer(i) => i.to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::LeftParen => "(".to_string(),
            TokenKind::RightParen => ")".to_string(),
            TokenKind::LeftBrace => "{".to_string(),
            TokenKind::RightBrace => "}".to_string(),
            TokenKind::Function => "function".to_string(),
            TokenKind::Let => "let".to_string(),
            TokenKind::Assign => "=".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Equal => "==".to_string(),
            TokenKind::Inequal => "!=".to_string(),
            TokenKind::Negation => "!".to_string(),
            TokenKind::LessThen => "<".to_string(),
            TokenKind::GreaterThen => ">".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Asterisk => "*".to_string(),
            TokenKind::True => "true".to_string(),
            TokenKind::False => "false".to_string(),
            TokenKind::If => "if".to_string(),
            TokenKind::Else => "else".to_string(),
            TokenKind::Return => "return".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::LeftBracket => "[".to_string(),
            TokenKind::RightBracket => "]".to_string(),
            TokenKind::Collon => ":".to_string(),
        }
    }
}
impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}

impl From<&TokenKind> for PureTokenKind {
    fn from(token_kind: &TokenKind) -> Self {
        match token_kind {
            TokenKind::Invalid(_) => PureTokenKind::Invalid,
            TokenKind::Identifier(_) => PureTokenKind::Identifier,
            TokenKind::StringLiteral(_) => PureTokenKind::StringLiteral,
            TokenKind::Integer(_) => PureTokenKind::Integer,
            TokenKind::Comma => PureTokenKind::Comma,
            TokenKind::Semicolon => PureTokenKind::Semicolon,
            TokenKind::LeftParen => PureTokenKind::LeftParen,
            TokenKind::RightParen => PureTokenKind::RightParen,
            TokenKind::LeftBrace => PureTokenKind::LeftBrace,
            TokenKind::RightBrace => PureTokenKind::RightBrace,
            TokenKind::Function => PureTokenKind::Function,
            TokenKind::Let => PureTokenKind::Let,
            TokenKind::Assign => PureTokenKind::Assign,
            TokenKind::Plus => PureTokenKind::Plus,
            TokenKind::Equal => PureTokenKind::Equal,
            TokenKind::Inequal => PureTokenKind::Inequal,
            TokenKind::Negation => PureTokenKind::Negation,
            TokenKind::LessThen => PureTokenKind::LessThen,
            TokenKind::GreaterThen => PureTokenKind::GreaterThen,
            TokenKind::Slash => PureTokenKind::Slash,
            TokenKind::Asterisk => PureTokenKind::Asterisk,
            TokenKind::True => PureTokenKind::True,
            TokenKind::False => PureTokenKind::False,
            TokenKind::If => PureTokenKind::If,
            TokenKind::Else => PureTokenKind::Else,
            TokenKind::Return => PureTokenKind::Return,
            TokenKind::Minus => PureTokenKind::Minus,
            TokenKind::LeftBracket => PureTokenKind::LeftBracket,
            TokenKind::RightBracket => PureTokenKind::RightBracket,
            TokenKind::Collon => PureTokenKind::Collon,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PureTokenKind {
    Invalid,
    Identifier,
    StringLiteral,
    Integer,

    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,

    Assign,
    Plus,
    Minus,
    Equal,
    Inequal,
    Negation,
    LessThen,
    GreaterThen,
    Slash,
    Asterisk,

    True,
    False,
    If,
    Else,
    Return,
    LeftBracket,
    RightBracket,
    Collon,
}
