use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::{ParsingError, ParsingErrorKind, Statement};

pub(crate) fn expect_indetifier<T>(
    tokens: &mut T,
    parent_token: &Token,
) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(aaa) => {
            if let Identifier(_) = aaa.kind() {
            } else {
                return Result::Err(ParsingError {
                    message: ParsingErrorKind::ExpectedIdentifier,
                    line: aaa.line(),
                    column: aaa.column(),
                });
            }
            Ok(Statement::IdentifierExpression { token: aaa })
        }
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedIdentifier,
            line: parent_token.line(),
            column: parent_token.column(),
        }),
    }
}
