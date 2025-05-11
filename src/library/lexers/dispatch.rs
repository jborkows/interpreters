use crate::tokens::Token;

use super::{
    idle::idle_parsing, parsing_states::LexerState, reading_equality::reading_equality,
    reading_negation::reading_negation,
};

pub(super) fn dispatch(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    return match state {
        LexerState::Idle => idle_parsing(line_number, column_number, character),
        LexerState::ReadingEquality {
            starting_position: _,
        } => reading_equality(line_number, column_number, character, state),
        LexerState::ReadingNegation {
            starting_position: _,
        } => reading_negation(line_number, column_number, character, state),
        _ => (LexerState::Idle, vec![]),
    };
}
