use crate::lexers::Token;
use crate::lexers::TokenKind::*;

use super::types::Expression;
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
            if let Identifier(name) = aaa.kind() {
                Ok(Statement::ExpressionStatement {
                    token: aaa.clone(),
                    expression: Box::new(Expression::IdentifierExpression {
                        name: name.to_string(),
                    }),
                })
            } else {
                return Result::Err(ParsingError {
                    message: ParsingErrorKind::ExpectedIdentifier,
                    line: aaa.line(),
                    column: aaa.column(),
                });
            }
        }
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedIdentifier,
            line: parent_token.line(),
            column: parent_token.column(),
        }),
    }
}
