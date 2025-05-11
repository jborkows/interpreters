use crate::{
    lexers::dispatch::dispatch,
    tokens::{Token, TokenKind},
};

use super::parsing_states::LexerState;

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
                        crate::lines::TokenPosition::from_range(
                            starting_position.line_number.0,
                            starting_position.column_number.0,
                            line_number,
                            column_number,
                        ),
                        crate::tokens::TokenKind::Inequal,
                    )],
                );
            }

            _ => {
                let mut tokens = vec![Token::new(
                    crate::lines::TokenPosition::single_character(
                        starting_position.line_number,
                        starting_position.column_number,
                    ),
                    TokenKind::Negation,
                )];
                let result = dispatch(line_number, column_number, character, &LexerState::Idle);

                tokens.extend(result.1);
                return (result.0, tokens);
            }
        },
        _ => unreachable!(),
    }
}
