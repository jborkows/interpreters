use crate::{
    lexers::dispatch::dispatch,
    tokens::{Token, TokenKind},
};

use super::{parsers::acceptable_separator, parsing_states::LexerState};

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
            negative,
        } => match character {
            '0'..='9' => {
                let next_digit = character.to_digit(10).unwrap();
                let new_value = *value * 10 + next_digit as i32;
                return (
                    LexerState::ReadingNumber {
                        starting_position: *starting_position,
                        value: new_value,
                        negative: *negative,
                    },
                    vec![],
                );
            }

            character if acceptable_separator(&character) => {
                let multipier = if *negative { -1 } else { 1 };
                let mut tokens = vec![Token::new(
                    starting_position.token_ends_with(line_number, column_number - 1),
                    TokenKind::Integer(multipier * *value as i32),
                )];
                let result = dispatch(line_number, column_number, character, &LexerState::Idle);
                tokens.extend(result.1);
                return (result.0, tokens);
            }

            _ => {
                return (
                    LexerState::ReadingInvalid {
                        starting_position: *starting_position,
                        reason: format!("Unexpected character '{}' in number", character),
                    },
                    vec![],
                );
            }
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
            negative,
        } => {
            let multipier = if *negative { -1 } else { 1 };
            let token = Token::new(
                starting_position.token_ends_with(line_number, column_number),
                TokenKind::Integer(multipier * *value as i32),
            );
            return Some(token);
        }
        _ => unreachable!(),
    }
}
