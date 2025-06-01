use crate::{
    lines::TokenPosition,
    tokens::{Token, TokenKind},
};

use super::{parsers::delegate_to_next, parsing_states::LexerState};

pub(super) fn reading_equality(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingEquality { starting_position } => match character {
            '=' => (
                LexerState::Idle,
                vec![Token::new(
                    starting_position.token_ends_with(line_number, column_number),
                    crate::tokens::TokenKind::Equal,
                )],
            ),

            _ => delegate_to_next(
                character,
                column_number,
                line_number,
                TokenKind::Assign,
                || {
                    TokenPosition::single_character(
                        starting_position.line_number,
                        starting_position.column_number,
                    )
                },
            ),
        },
        _ => unreachable!(),
    }
}

pub(super) fn finish_equality(state: &LexerState) -> Option<Token> {
    match state {
        LexerState::ReadingEquality { starting_position } => {
            let token = Token::new(
                TokenPosition::single_character(
                    starting_position.line_number,
                    starting_position.column_number,
                ),
                TokenKind::Assign,
            );
            Some(token)
        }
        _ => unreachable!(),
    }
}
