use crate::{
    lexers::dispatch::dispatch,
    tokens::{Token, TokenKind},
};

use super::{parsers::delegate_to_next, parsing_states::LexerState};

pub(super) fn reading_negation(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingNegation { starting_position } => match character {
            '=' => {
                return (
                    LexerState::Idle,
                    vec![Token::new(
                        starting_position.token_ends_with(line_number, column_number),
                        crate::tokens::TokenKind::Inequal,
                    )],
                );
            }

            _ => {
                return delegate_to_next(
                    character,
                    column_number,
                    line_number,
                    TokenKind::Negation,
                    || {
                        crate::lines::TokenPosition::single_character(
                            starting_position.line_number,
                            starting_position.column_number,
                        )
                    },
                );
            }
        },
        _ => unreachable!(),
    }
}

pub(super) fn finish_negation(state: &LexerState) -> Option<Token> {
    match state {
        LexerState::ReadingNegation { starting_position } => {
            let token = Token::new(
                crate::lines::TokenPosition::single_character(
                    starting_position.line_number,
                    starting_position.column_number,
                ),
                TokenKind::Invalid(String::from("Single ! at end is not valid")),
            );
            return Some(token);
        }
        _ => unreachable!(),
    }
}
