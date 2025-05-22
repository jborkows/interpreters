use crate::tokens::Token;

use super::{base::Node, expression::Expression, expression::Identifier};

pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
        value: Box<dyn Expression>,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { name, value, .. } => {
                format!("let {}={}", name.token_literal(), value.token_literal())
            }
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
