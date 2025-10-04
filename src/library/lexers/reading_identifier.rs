use crate::{
    lines::TokenPosition,
    tokens::{Token, TokenKind},
};

use super::{parsers::delegate_to_next_position, parsing_states::LexerState};

pub(super) fn reading_identifier(
    line_number: u16,
    column_number: u16,
    character: char,
    state: &LexerState,
) -> (LexerState, Vec<Token>) {
    match state {
        LexerState::ReadingIdentifier {
            starting_position,
            chars,
        } => match character {
            character if character.is_alphanumeric() => {
                let mut chars = chars.clone();
                chars.push(character);
                (
                    LexerState::ReadingIdentifier {
                        starting_position: *starting_position,
                        chars,
                    },
                    vec![],
                )
            }
            _ => {
                let result = finish_it(line_number, column_number - 1, state);
                delegate_to_next_position(character, column_number, line_number, result.0, result.1)
            }
        },
        _ => unreachable!(),
    }
}

fn finish_it(
    line_number: u16,
    column_number: u16,
    state: &LexerState,
) -> (TokenKind, TokenPosition) {
    match state {
        LexerState::ReadingIdentifier {
            starting_position,
            chars,
        } => {
            let text: String = chars.iter().collect();
            let as_ref: &str = &text;
            let token_kind = match as_ref {
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "let" => TokenKind::Let,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "return" => TokenKind::Return,
                "fn" => TokenKind::Function,
                "macro" => TokenKind::Macro,
                _ => TokenKind::Identifier(text),
            };
            let position = starting_position.token_ends_with(line_number, column_number);
            (token_kind, position)
        }
        _ => unreachable!(),
    }
}

pub(super) fn identifier_end_of_line(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> (Option<LexerState>, Vec<Token>) {
    let result = finish_it(line_number, column_number, state);
    (Some(LexerState::Idle), vec![Token::new(result.1, result.0)])
}

pub(super) fn finish_identifier(
    state: &LexerState,
    line_number: u16,
    column_number: u16,
) -> Option<Token> {
    match state {
        LexerState::ReadingIdentifier {
            starting_position: _,
            chars: _,
        } => {
            let result = finish_it(line_number, column_number, state);
            Some(Token::new(result.1, result.0))
        }
        _ => unreachable!(),
    }
}
