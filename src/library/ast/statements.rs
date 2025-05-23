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

impl Node for Statement {}

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

impl Node for Program {}
impl ToString for Program {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for statement in &self.statements {
            result.push_str(&statement.to_string());
            result.push('\n');
        }
        result
    }
}
