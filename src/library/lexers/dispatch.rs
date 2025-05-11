use crate::tokens::Token;

use super::{
    idle::idle_parsing,
    parsing_states::LexerState,
    reading_equality::{finish_equality, reading_equality},
    reading_invalid::{finish_invalid, reading_invalid},
    reading_minus::{finish_minus, reading_minus},
    reading_negation::{finish_negation, reading_negation},
    reading_number::{finish_number, reading_number},
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
        LexerState::ReadingInvalid {
            starting_position: _,
            reason: _,
        } => reading_invalid(line_number, column_number, character, state),
        LexerState::ReadingMinus {
            starting_position: _,
        } => reading_minus(character, state),
        LexerState::ReadingNumber {
            starting_position: _,
            value: _,
            negative: _,
        } => reading_number(line_number, column_number, character, state),
        _ => (LexerState::Idle, vec![]),
    };
}

pub(super) fn finish_it(state: &LexerState, line_number: u16, column_number: u16) -> Option<Token> {
    match state {
        LexerState::Idle => None,
        LexerState::ReadingEquality {
            starting_position: _,
        } => finish_equality(state),
        LexerState::ReadingNegation {
            starting_position: _,
        } => finish_negation(state),
        LexerState::ReadingMinus {
            starting_position: _,
        } => finish_minus(state),
        LexerState::ReadingNumber {
            starting_position: _,
            value: _,
            negative: _,
        } => finish_number(state, line_number, column_number),
        LexerState::ReadingInvalid {
            starting_position: _,
            reason: _,
        } => finish_invalid(state, line_number, column_number),
    }
}
