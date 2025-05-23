use crate::tokens::{Token, TokenKind};

use super::parsing_states::LexerState;

pub(super) fn reading_text(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingText {
            starting_position,
            chars,
        } => match character {
            '"' => {
                return (
                    LexerState::Idle,
                    vec![Token::new(
                        starting_position.token_ends_with(line_number, column_number),
                        TokenKind::StringLiteral(chars.borrow_mut().iter().collect()),
                    )],
                );
            }
            _ => {
                chars.borrow_mut().push(character);
                return (
                    LexerState::ReadingText {
                        starting_position: *starting_position,
                        chars: chars.clone(),
                    },
                    vec![],
                );
            }
        },
        _ => unreachable!(),
    }
}

pub(super) fn text_end_of_line(state: &LexerState) -> (Option<LexerState>, Vec<Token>) {
    match state {
        LexerState::ReadingText {
            starting_position,
            chars,
        } => {
            chars.borrow_mut().push('\n');
            return (
                Some(LexerState::ReadingText {
                    starting_position: *starting_position,
                    chars: chars.clone(),
                }),
                vec![],
            );
        }
        _ => unreachable!(),
    }
}

pub(super) fn finish_text(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> Option<Token> {
    match state {
        LexerState::ReadingText {
            starting_position,
            chars: _,
        } => {
            let token = Token::new(
                starting_position.token_ends_with(line_number, column_number),
                TokenKind::Invalid(String::from("Unclosed string literal")),
            );
            return Some(token);
        }
        _ => unreachable!(),
    }
}
