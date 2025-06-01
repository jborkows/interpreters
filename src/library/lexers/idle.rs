use std::{cell::RefCell, rc::Rc};

use crate::{lines::TextPosition, tokens::Token};

use super::{parsers::read_special_character, parsing_states::LexerState};

pub(super) fn idle_parsing(
    line_number: u16,
    column_number: u16,
    character: char,
) -> (LexerState, Vec<Token>) {
    let text_possition = TextPosition::new(line_number, column_number);
    let maybe_state = match character {
        '=' => Some(LexerState::ReadingEquality {
            starting_position: text_possition,
        }),

        '!' => Some(LexerState::ReadingNegation {
            starting_position: text_possition,
        }),

        ch if ch.is_numeric() => Some(LexerState::ReadingNumber {
            starting_position: text_possition,
            value: ch.to_digit(10).unwrap(),
        }),
        ch if ch.is_alphabetic() => Some(LexerState::ReadingIdentifier {
            starting_position: text_possition,
            chars: vec![character],
        }),

        '"' => Some(LexerState::ReadingText {
            starting_position: text_possition,
            chars: Rc::new(RefCell::new(vec![])),
        }),

        _ => None,
    };
    if let Some(state) = maybe_state {
        return (state, vec![]);
    }

    match read_special_character(character) {
        Some(token_kind) => {
            let position = crate::lines::TextPosition {
                line_number: crate::lines::LineNumber(line_number),
                column_number: crate::lines::ColumnNumber(column_number),
            };
            let token_position = crate::lines::TokenPosition::new(position, position);
            let token = Token::new(token_position, token_kind);
            (LexerState::Idle, vec![token])
        }
        None => {
            // Handle the case where the character is not a special character
            (LexerState::Idle, vec![])
        }
    }
}
