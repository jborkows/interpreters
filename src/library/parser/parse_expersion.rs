use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::{ParsingError, ParsingErrorKind, Statement};

use super::types::Statement::*;

pub(crate) fn parse_expression<'a, T>(
    tokens: &mut T,
    parent_token: &Token,
) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(Token(_, _, Integer(value))) => Ok(Statement::LiteralInt {
            token: token.unwrap(),
            value,
        }),
        Some(Token(line, column, _)) => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedInteger,
            line,
            column,
        }),
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedInteger,
            line: parent_token.line(),
            column: parent_token.column(),
        }),
    }
}
