use std::{any::Any, fmt::Display, rc::Rc};

use crate::{
    join_collection, join_rc_collection,
    tokens::{Token, TokenKind},
};

use super::{base::Node, statements::Statement};

#[derive(Debug)]
pub enum Expression {
    Identifier(Rc<Token>),
    IntegerLiteral(Rc<Token>),
    PrefixOperator {
        token: Rc<Token>,
        operator: PrefixOperatorType,
        right: Box<Expression>,
    },
    InfixExpression {
        token: Rc<Token>,
        left: Box<Expression>,
        operator: InfixOperatorType,
        right: Box<Expression>,
    },
    CallExpression {
        token: Rc<Token>,
        function: Box<Expression>, //Identifier or FunctionLiteral
        arguments: Vec<Expression>,
    },
    BooleanLiteral {
        token: Rc<Token>,
        value: bool,
    },
    StringLiteral(Rc<Token>),
    IfExpression {
        #[allow(dead_code)]
        token: Rc<Token>,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    FunctionLiteral {
        token: Rc<Token>,
        parameters: Rc<Vec<Expression>>, // Identifier
        body: Box<Statement>,
    },
}

impl Node for Expression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(token) => {
                let real_type = token.as_ref();
                match &real_type.kind {
                    TokenKind::Identifier(s) => write!(f, "{}", s),
                    _ => panic!("Invalid token type for Identifier: {:?}", real_type),
                }
            }
            Expression::IntegerLiteral(token) => {
                let real_type = token.as_ref();
                match &real_type.kind {
                    TokenKind::Integer(i) => write!(f, "{}", i),
                    _ => panic!("Invalid token type for IntegerLiteral: {:?}", real_type),
                }
            }
            Expression::PrefixOperator {
                token: _,
                operator,
                right,
            } => write!(f, "({}{})", operator, right),
            Expression::InfixExpression {
                token: _,
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::CallExpression {
                token: _,
                function,
                arguments,
            } => {
                let args = join_collection!(arguments, ", ");
                write!(f, "{}({})", function, args)
            }
            Expression::BooleanLiteral { token: _, value } => write!(f, "{}", value),
            Expression::StringLiteral(token) => {
                let real_type = token.as_ref();
                match &real_type.kind {
                    TokenKind::StringLiteral(s) => write!(f, "{}", s),
                    _ => panic!("Invalid token type for StringLiteral: {:?}", real_type),
                }
            }
            Expression::IfExpression {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                let mut result = format!("if ({}){{", condition);
                result.push_str(&consequence.to_string());
                result.push('}');
                if let Some(alt) = &alternative {
                    result.push_str("else {");
                    result.push_str(&alt.to_string());
                    result.push('}');
                }
                write!(f, "{}", result)
            }
            Expression::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => {
                let params = join_rc_collection!(parameters, ", ");
                write!(f, "fn({}){{ {} }}", params, body)
            }
        }
    }
}

pub fn if_expression(
    token: Rc<Token>,
    condition: Expression,
    consequence: Statement,
    alternative: Option<Statement>,
) -> Expression {
    match consequence {
        Statement::BlockStatement { .. } => {}
        _ => {
            panic!("Consequence must be a BlockStatement");
        }
    }
    match alternative {
        Some(Statement::BlockStatement { .. }) => {}
        None => {}
        _ => {
            panic!("Alternative must be a BlockStatement");
        }
    }
    Expression::IfExpression {
        token,
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: alternative.map(Box::new),
    }
}
pub fn function_literal(
    token: Rc<Token>,
    parameters: Rc<Vec<Expression>>,
    body: Statement,
) -> Expression {
    match body {
        Statement::BlockStatement { .. } => {}
        _ => {
            panic!("Body must be a BlockStatement");
        }
    }
    parameters.iter().for_each(|param| {
        if !matches!(param, Expression::Identifier(_)) {
            panic!("Parameters must be Identifier expressions");
        }
    });
    Expression::FunctionLiteral {
        token,
        parameters,
        body: Box::new(body),
    }
}

pub fn identifier(token: Rc<Token>) -> Expression {
    match token.as_ref().kind {
        TokenKind::Identifier(_) => Expression::Identifier(token),
        _ => panic!("Invalid token type for Identifier: {:?}", token),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PrefixOperatorType {
    Bang,
    Minus,
}

impl Display for PrefixOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperatorType::Bang => write!(f, "!"),
            PrefixOperatorType::Minus => write!(f, "-"),
        }
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

impl Display for InfixOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperatorType::Plus => write!(f, "+"),
            InfixOperatorType::Minus => write!(f, "-"),
            InfixOperatorType::NotEqual => write!(f, "!="),
            InfixOperatorType::Multiply => write!(f, "*"),
            InfixOperatorType::Divide => write!(f, "/"),
            InfixOperatorType::LessThan => write!(f, "<"),
            InfixOperatorType::GreaterThan => write!(f, ">"),
            InfixOperatorType::Equal => write!(f, "=="),
        }
    }
}
