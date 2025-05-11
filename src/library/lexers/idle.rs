use crate::{lines::TextPosition, tokens::Token};

use super::{parsers::read_special_character, parsing_states::LexerState};

pub(super) fn idle_parsing(
    line_number: u16,
    column_number: u16,
    character: char,
) -> (LexerState, Vec<Token>) {
    match character {
        '=' => {
            return (
                LexerState::ReadingEquality {
                    starting_position: TextPosition::new(line_number, column_number),
                },
                vec![],
            );
        }

        '!' => {
            return (
                LexerState::ReadingNegation {
                    starting_position: TextPosition::new(line_number, column_number),
                },
                vec![],
            );
        }

        _ => {}
    }

    return match read_special_character(character) {
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
    };
}
