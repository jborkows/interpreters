use super::{
    base::{SourceCharecter, State},
    processors::{operator, sign},
    tokens::Token,
};

pub(crate) fn read_idle(charecter: &SourceCharecter) -> (State, Vec<Token>) {
    if charecter.is_whitespace() || charecter.ch == '\0' {
        return (State::Idle, vec![]);
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return (charecter.as_reading_operator(), vec![]);
    }
    let sign = sign(&charecter);
    if sign.is_some() {
        return (State::Idle, vec![Token::new(charecter, sign.unwrap())]);
    }
    if charecter.ch.is_ascii_alphabetic() {
        return (charecter.as_reading_text(), vec![]);
    }
    if charecter.ch.is_ascii_digit() {
        return (charecter.as_reading_number(), vec![]);
    }

    todo!()
}
