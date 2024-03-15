use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::{ParsingError, ParsingErrorKind, Program};

use super::parse_let::parse_let_statement;
use super::parse_return::parse_return_statement;

pub fn parse<T>(mut tokens: T) -> Program
where
    T: Iterator<Item = Token>,
{
    let mut program = Program::new();

    while let Some(token) = tokens.next() {
        match token.kind() {
            Let() => {
                let statement = parse_let_statement(&mut tokens, token);
                program.push(statement);
            }
            Return() => {
                let statement = parse_return_statement(&mut tokens, token);
                program.push(statement);
            }
            Semicolon() => {}
            _ => program.push(Result::Err(ParsingError {
                message: ParsingErrorKind::UnexpectedToken(token.kind().clone()),
                line: token.line(),
                column: token.column(),
            })),
        }
    }
    program
}
