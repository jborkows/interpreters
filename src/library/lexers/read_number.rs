use super::{
    base::{SourceCharecter, State, StateLineContext},
    processors::{operator, sign},
    tokens::{Token, TokenKind},
};

pub(crate) fn read_number(
    charecter: SourceCharecter,
    context: StateLineContext,
) -> (State, Vec<Token>) {
    if charecter.is_whitespace() || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![context.full_token(context.read_text_as_number())],
        );
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            charecter.as_reading_operator(),
            vec![context.full_token(context.read_text_as_number())],
        );
    }

    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                context.full_token(context.read_text_as_number()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    if charecter.ch.is_ascii_digit() {
        return (
            State::ReadingNumber(StateLineContext {
                text: context.text + &charecter.ch.to_string(),
                ..context
            }),
            vec![],
        );
    }
    return (charecter.as_reading_text(), vec![]);
}
