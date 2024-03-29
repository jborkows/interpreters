use crate::lexers::{base::State, processors::operator, processors::sign, tokens::Token};

use super::{
    base::{SourceCharecter, StateLineContext},
    tokens::TokenKind,
};

pub(crate) fn read_operator(
    charecter: SourceCharecter,
    state: StateLineContext,
) -> (State, Vec<Token>) {
    if charecter.is_whitespace() || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![state.full_token(state.read_text_as_operator())],
        );
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return fetch_operator(&charecter, &state);
    }
    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                state.full_token(state.read_text_as_operator()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    if charecter.ch.is_ascii_alphabetic() {
        return (
            charecter.as_reading_text(),
            vec![Token::new(&charecter, state.read_text_as_operator())],
        );
    }
    if charecter.ch.is_ascii_digit() {
        return (
            charecter.as_reading_number(),
            vec![Token::new(&charecter, state.read_text_as_operator())],
        );
    }
    todo!()
}

fn fetch_operator(charecter: &SourceCharecter, state: &StateLineContext) -> (State, Vec<Token>) {
    match format!("{}{}", state.text, &charecter.ch.to_string()).as_str() {
        "==" => (State::Idle, vec![state.full_token(TokenKind::Equality())]),
        "!=" => (State::Idle, vec![state.full_token(TokenKind::Inequality())]),
        _ => (
            State::ReadingOperator(StateLineContext {
                text: String::from(charecter.ch),
                line: charecter.line_number,
                column: charecter.column_number,
            }),
            vec![state.full_token(state.read_text_as_operator())],
        ),
    }
}
