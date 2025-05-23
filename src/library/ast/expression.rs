use std::{any::Any, rc::Rc};

use crate::tokens::{Token, TokenKind};

use super::base::Node;

pub(crate) trait Expression: Node + ToString + Any {
    fn as_any(&self) -> &dyn Any;
}

pub(crate) struct Identifier {
    pub token: Rc<Token>,
}

impl Node for Identifier {}
impl Expression for Identifier {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ToString for Identifier {
    fn to_string(&self) -> String {
        let real_type = self.token.as_ref();
        match &real_type.kind {
            TokenKind::Identifier(s) => s.to_string(),
            _ => panic!("Invalid token type for Identifier: {:?}", real_type),
        }
    }
}

pub(crate) struct IntegerLiteral {
    pub token: Rc<Token>,
}
impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value().to_string()
    }
}

impl Node for IntegerLiteral {}
impl IntegerLiteral {
    pub fn value(&self) -> u32 {
        let real_type = self.token.as_ref();
        return match &real_type.kind {
            TokenKind::Integer(i) => *i,
            _ => panic!("Invalid token type for IntegerLiteral: {:?}", real_type),
        };
    }
}

impl Expression for IntegerLiteral {
    fn as_any(&self) -> &dyn Any {
        self
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
    pub token: Rc<Token>,
    pub operator: PrefixOperatorType,
    pub right: Box<dyn Expression>,
}
impl Node for PrefixOperator {}
impl Expression for PrefixOperator {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ToString for PrefixOperator {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator.to_string(), self.right.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum InfixOperatorType {
    Plus,
    Minus,
    NotEqual,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
}
impl ToString for InfixOperatorType {
    fn to_string(&self) -> String {
        match self {
            InfixOperatorType::Plus => "+".to_string(),
            InfixOperatorType::Minus => "-".to_string(),
            InfixOperatorType::NotEqual => "!=".to_string(),
            InfixOperatorType::Multiply => "*".to_string(),
            InfixOperatorType::Divide => "/".to_string(),
            InfixOperatorType::LessThan => "<".to_string(),
            InfixOperatorType::GreaterThan => ">".to_string(),
            InfixOperatorType::Equal => "==".to_string(),
        }
    }
}

pub(crate) struct InfixExpression {
    pub token: Rc<Token>,
    pub left: Box<dyn Expression>,
    pub operator: InfixOperatorType,
    pub right: Box<dyn Expression>,
}
impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator.to_string(),
            self.right.to_string()
        )
    }
}
impl Node for InfixExpression {}
impl Expression for InfixExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
