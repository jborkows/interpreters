use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{expression::Expression, statements::Statement},
    object::{Environment, Identifier, Object, error_at},
    tokens::{Token, TokenKind},
};

pub fn function_literal_evaluation(
    token: &Token,
    parameters: &Vec<Expression>,
    body: &Statement,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let mut parsed_parameters: Vec<Identifier> = vec![];
    for parameter in parameters {
        match parameter {
            Expression::Identifier(id_token) => match &id_token.kind {
                TokenKind::Identifier(value) => {
                    parsed_parameters.push(Identifier {
                        name: value.clone(),
                    });
                }
                _ => return error_at("Function parameters must be identifiers.", token),
            },
            _ => return error_at("Function parameters must be identifiers.", token),
        }
    }
    match body {
        Statement::Block { token, statements } => Rc::new(Object::Function {
            parameters: parsed_parameters,
            body: Rc::new(Statement::Block {
                token: token.clone(),
                statements: statements.clone(),
            }),
            env: env.clone(),
        }),
        _ => error_at("Function body must be a block statement.", token),
    }
}
