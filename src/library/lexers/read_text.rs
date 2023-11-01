use super::{
    base::{SourceCharecter, State, StateLineContext},
    processors::{operator, sign},
    tokens::{Token, TokenKind},
};

pub(crate) fn read_text(
    charecter: SourceCharecter,
    context: StateLineContext,
) -> (State, Vec<Token>) {
    if charecter.is_whitespace() || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![context.full_token(IdentifierString(&context.text).as_kind())],
        );
    }

    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            charecter.as_reading_operator(),
            vec![context.full_token(IdentifierString(&context.text).as_kind())],
        );
    }

    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                context.full_token(IdentifierString(&context.text).as_kind()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    return (
        State::ReadingText(StateLineContext {
            text: context.text + &charecter.ch.to_string(),
            ..context
        }),
        vec![],
    );
}

struct IdentifierString<'a>(&'a String);
impl IdentifierString<'_> {
    fn as_kind(&self) -> TokenKind {
        match self.0.as_str() {
            "let" => TokenKind::Let(),
            "fn" => TokenKind::Function(),
            // "true" => TokenKind::True(),
            // "false" => TokenKind::False(),
            // "if" => TokenKind::If(),
            // "else" => TokenKind::Else(),
            // "return" => TokenKind::Return(),
            _ => TokenKind::Identifier(self.0.clone()),
        }
    }
}
