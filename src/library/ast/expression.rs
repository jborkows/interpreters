use crate::tokens::Token;

use super::base::Node;

pub(crate) trait Expression: Node {
    fn expression_kind(&self) -> ExpressionKind;
}

pub(crate) struct Identifier {
    pub token: Token,
    pub value: String,
}

pub(crate) struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionKind {
    Identifier,
    IntegerLiteral,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.short()
    }
}
impl Expression for Identifier {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::Identifier
    }
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.short()
    }
}
impl Expression for IntegerLiteral {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::IntegerLiteral
    }
}
