use crate::tokens::{Token, TokenKind};

use super::parsing_states::LexerState;

pub(super) fn reading_minus(character: char, state: &LexerState) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingMinus { starting_position } => match character {
            character if character.is_numeric() => {
                return (
                    LexerState::ReadingNumber {
                        starting_position: *starting_position,
                        value: character.to_digit(10).unwrap() as i32,
                        negative: true,
                    },
                    vec![],
                );
            }

            _ => {
                return (
                    LexerState::ReadingInvalid {
                        starting_position: *starting_position,
                        reason: String::from("Only numbers are allowed after '-'"),
                    },
                    vec![],
                );
            }
        },
        _ => unreachable!(),
    }
}
pub(super) fn finish_minus(state: &LexerState) -> Option<Token> {
    match state {
        LexerState::ReadingMinus { starting_position } => {
            let token = Token::new(
                crate::lines::TokenPosition::single_character(
                    starting_position.line_number,
                    starting_position.column_number,
                ),
                TokenKind::Invalid(String::from("Single - at end is not valid")),
            );
            return Some(token);
        }
        _ => unreachable!(),
    }
}
