use crate::tokens::{Token, TokenKind};

use super::{
    parsers::{acceptable_separator, delegate_to_next},
    parsing_states::LexerState,
};

pub(super) fn reading_invalid(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingInvalid {
            starting_position,
            reason,
        } => match character {
            character if acceptable_separator(&character) => {
                return delegate_to_next(
                    character,
                    column_number,
                    line_number,
                    TokenKind::Invalid(reason.clone()),
                    || starting_position.token_ends_with(line_number, column_number - 1),
                );
            }

            _ => {
                return (
                    LexerState::ReadingInvalid {
                        starting_position: *starting_position,
                        reason: reason.clone(),
                    },
                    vec![],
                );
            }
        },
        _ => unreachable!(),
    }
}

pub(super) fn finish_invalid(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> Option<Token> {
    match state {
        LexerState::ReadingInvalid {
            starting_position,
            reason,
        } => {
            let token = Token::new(
                starting_position.token_ends_with(line_number, column_number),
                TokenKind::Invalid(reason.clone()),
            );
            return Some(token);
        }
        _ => unreachable!(),
    }
}
