use crate::tokens::{Token, TokenKind};

use super::{parsers::delegate_to_next, parsing_states::LexerState};

pub(super) fn reading_number(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingNumber {
            starting_position,
            value,
        } => match character {
            '0'..='9' => {
                let next_digit = character.to_digit(10).unwrap();
                let new_value = *value * 10 + next_digit;
                (
                    LexerState::ReadingNumber {
                        starting_position: *starting_position,
                        value: new_value,
                    },
                    vec![],
                )
            }

            character if character.is_alphabetic() || character == '_' => (
                LexerState::ReadingInvalid {
                    starting_position: *starting_position,
                    reason: format!("Unexpected character '{}' in number", character),
                },
                vec![],
            ),

            _ => delegate_to_next(
                character,
                column_number,
                line_number,
                TokenKind::Integer(*value),
                || starting_position.token_ends_with(line_number, column_number - 1),
            ),
        },
        _ => unreachable!(),
    }
}

pub(super) fn finish_number(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> Option<Token> {
    match state {
        LexerState::ReadingNumber {
            starting_position,
            value,
        } => {
            let token = Token::new(
                starting_position.token_ends_with(line_number, column_number),
                TokenKind::Integer(*value),
            );
            Some(token)
        }
        _ => unreachable!(),
    }
}
