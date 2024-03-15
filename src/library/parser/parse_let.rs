use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::{ParsingError, ParsingErrorKind, Statement};

use super::base::expect_indetifier;
use super::parse_expersion::parse_expression;
use super::types::Statement::*;
pub(crate) fn parse_let_statement<T>(
    tokens: &mut T,
    token: Token,
) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let name = expect_indetifier(tokens, &token)?;
    match tokens.next() {
        Some(Token(_, _, Assign())) => {
            let value = parse_expression(tokens, &token)?;
            Ok(LetStatement {
                token,
                name: Box::new(name),
                value: Box::new(value),
            })
        }
        Some(Token(line, column, kind)) => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedAssign(Some(kind)),
            line,
            column,
        }),
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedAssign(None),
            line: token.line(),
            column: token.column(),
        }),
    }
}
