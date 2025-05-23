use crate::tokens::Token;

use super::{
    idle::idle_parsing,
    parsing_states::LexerState,
    reading_equality::{finish_equality, reading_equality},
    reading_identifier::{finish_identifier, identifier_end_of_line, reading_identifier},
    reading_invalid::{finish_invalid, reading_invalid},
    reading_negation::{finish_negation, reading_negation},
    reading_number::{finish_number, reading_number},
    reading_text::{finish_text, reading_text, text_end_of_line},
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
        LexerState::ReadingNumber {
            starting_position: _,
            value: _,
        } => reading_number(line_number, column_number, character, state),
        LexerState::ReadingText {
            starting_position: _,
            chars: _,
        } => reading_text(line_number, column_number, character, state),
        LexerState::ReadingIdentifier {
            starting_position: _,
            chars: _,
        } => reading_identifier(line_number, column_number, character, state),
    };
}

pub(super) fn end_of_line(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> (Option<LexerState>, Vec<Token>) {
    return match state {
        LexerState::ReadingText {
            starting_position: _,
            chars: _,
        } => text_end_of_line(state),
        LexerState::ReadingIdentifier {
            starting_position: _,
            chars: _,
        } => identifier_end_of_line(state, line_number, column_number),
        _ => (None, vec![]),
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
        LexerState::ReadingNumber {
            starting_position: _,
            value: _,
        } => finish_number(state, line_number, column_number),
        LexerState::ReadingText {
            starting_position: _,
            chars: _,
        } => finish_text(state, line_number, column_number),
        LexerState::ReadingInvalid {
            starting_position: _,
            reason: _,
        } => finish_invalid(state, line_number, column_number),
        LexerState::ReadingIdentifier {
            starting_position: _,
            chars: _,
        } => finish_identifier(state, line_number, column_number),
    }
}
