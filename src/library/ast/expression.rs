use crate::tokens::Token;

use super::base::Node;

pub(crate) trait Expression: Node + ToString {
    fn expression_kind(&self) -> ExpressionKind;
}

pub(crate) struct Identifier {
    pub token: Token,
    pub value: String,
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
impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

pub(crate) struct IntegerLiteral {
    pub token: Token,
    pub value: u32,
}
impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PrefixOperatorType {
    Bang,
    Minus,
}
impl ToString for PrefixOperatorType {
    fn to_string(&self) -> String {
        match self {
            PrefixOperatorType::Bang => "!".to_string(),
            PrefixOperatorType::Minus => "-".to_string(),
        }
    }
}

pub(crate) struct PrefixOperator {
    pub token: Token,
    pub operator: PrefixOperatorType,
    pub right: Box<dyn Expression>,
}
impl Node for PrefixOperator {
    fn token_literal(&self) -> String {
        self.token.short()
    }
}
impl Expression for PrefixOperator {
    fn expression_kind(&self) -> ExpressionKind {
        ExpressionKind::PrefixOperator
    }
}
impl ToString for PrefixOperator {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator.to_string(), self.right.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionKind {
    Identifier,
    IntegerLiteral,
    PrefixOperator,
}
