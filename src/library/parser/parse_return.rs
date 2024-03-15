use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::{ParsingError, ParsingErrorKind, Statement};

use super::types::Statement::*;
pub(crate) fn parse_return_statement<T>(
    tokens: &mut T,
    token: Token,
) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(Token(line, column, Identifier(x))) => Ok(Statement::ReturnStatement {
            token,
            value: Box::new(IdentifierExpression {
                token: Token(line, column, Identifier(x)),
            }),
        }),
        Some(Token(line, column, Integer(x))) => Ok(Statement::ReturnStatement {
            token,
            value: Box::new(Statement::LiteralInt {
                value: x,
                token: Token(line, column, Integer(x)),
            }),
        }),
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::NotImplementedYet,
            line: token.line(),
            column: token.column(),
        }),
    }
}
