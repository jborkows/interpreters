use std::rc::Rc;

use crate::{join_collection, join_rc_collection, tokens::Token};

use super::{base::Node, expression::Expression};

#[derive(Debug)]
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
    ExpressionStatement {
        token: Rc<Token>,
        expression: Expression,
    },
    BlockStatement {
        token: Rc<Token>,
        statements: Rc<Vec<Statement>>,
    },
}

impl Node for Statement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let { name, value, .. } => {
                format!("let {}={}", name.to_string(), value.to_string())
            }
            Statement::Return {
                token,
                return_value,
            } => {
                format!("{} {}", token.short(), return_value.to_string())
            }
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => format!("{}", expression.to_string()),
            Statement::BlockStatement {
                token: _,
                statements,
            } => {
                format!("{}", join_rc_collection!(statements, " "))
            }
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl ToString for Program {
    fn to_string(&self) -> String {
        format!("{}", join_collection!(&self.statements, "\n"))
    }
}
