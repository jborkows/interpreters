use std::{fmt::Display, rc::Rc};

use crate::{join_collection, join_rc_collection, tokens::Token};

use super::{base::Node, expression::Expression};

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        token: Rc<Token>,
        name: Expression,
        value: Expression,
    },
    Return {
        token: Rc<Token>,
        return_value: Expression,
    },
    AExpression {
        #[allow(dead_code)]
        token: Rc<Token>,
        expression: Expression,
    },
    Block {
        token: Rc<Token>,
        statements: Rc<Vec<Statement>>,
    },
}

impl Node for Statement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value, .. } => {
                write!(f, "let {}={}", name, value)
            }
            Statement::Return {
                token,
                return_value,
            } => write!(f, "{} {}", token.short(), return_value),
            Statement::AExpression {
                token: _,
                expression,
            } => write!(f, "{}", expression),
            Statement::Block {
                token: _,
                statements,
            } => write!(f, "{}", join_rc_collection!(statements, "\n")),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_collection!(&self.statements, "\n"))
    }
}
