use super::{
    base::{SourceCharecter, State, StateLineContext},
    tokens::TokenKind,
};

pub(crate) fn operator(charecter: &SourceCharecter) -> Option<TokenKind> {
    match charecter.ch {
        '=' => Some(TokenKind::Assign()),
        '+' => Some(TokenKind::Plus()),
        _ => None,
    }
}
pub(crate) fn sign(charecter: &SourceCharecter) -> Option<TokenKind> {
    match charecter.ch {
        ',' => Some(TokenKind::Comma()),
        ';' => Some(TokenKind::Semicolon()),
        '(' => Some(TokenKind::LeftParen()),
        ')' => Some(TokenKind::RightParen()),
        '{' => Some(TokenKind::LeftBrace()),
        '}' => Some(TokenKind::RightBrace()),
        _ => None,
    }
}

impl StateLineContext {
    pub(crate) fn read_text_as_operator(&self) -> TokenKind {
        operator_token_from_text(self.text.as_str())
    }

    pub(crate) fn read_text_as_number(&self) -> TokenKind {
        TokenKind::Integer(self.text.parse().unwrap())
    }
}

pub(crate) fn operator_token_from_text(text: &str) -> TokenKind {
    let token = match text {
        "=" => TokenKind::Assign(),
        "+" => TokenKind::Plus(),
        _ => TokenKind::Illegal(format!("Unknown operator: {}", text)),
    };
    token
}

impl SourceCharecter {
    pub(crate) fn as_reading_operator(&self) -> State {
        let charecter = self;
        State::ReadingOperator(self.as_state_line_context())
    }
    pub(crate) fn as_reading_text(&self) -> State {
        let charecter = self;
        State::ReadingText(self.as_state_line_context())
    }
    pub(crate) fn as_reading_number(&self) -> State {
        let charecter = self;
        State::ReadingNumber(self.as_state_line_context())
    }
    pub(crate) fn as_state_line_context(&self) -> StateLineContext {
        let charecter = self;
        StateLineContext {
            text: String::from(charecter.ch),
            line: charecter.line_number,
            column: charecter.column_number,
        }
    }
}
