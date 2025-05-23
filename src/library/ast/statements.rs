use std::rc::Rc;

use crate::tokens::Token;

use super::{base::Node, expression::Expression, expression::Identifier};

pub enum Statement {
    Let {
        token: Rc<Token>,
        name: Identifier,
        value: Box<dyn Expression>,
    },
    Return {
        token: Rc<Token>,
        return_value: Box<dyn Expression>,
    },
    ExpressionStatement {
        token: Rc<Token>,
        expression: Box<dyn Expression>,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { name, value, .. } => {
                format!("let {}={}", name.token_literal(), value.token_literal())
            }
            Statement::Return {
                token,
                return_value,
            } => {
                format!("{} {}", token.short(), return_value.token_literal())
            }
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                format!("{}", expression.token_literal())
            }
        }
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
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::new()
        } else {
            self.statements[0].token_literal()
        }
    }
}
