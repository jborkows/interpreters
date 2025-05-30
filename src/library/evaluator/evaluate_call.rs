use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    end_flow,
    object::{Environment, Object, error_at},
    tokens::Token,
};

use super::evaluate_expression;

pub fn evaluate_call_expression(
    token: &Token,
    function: &Expression,
    arguments: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Object {
    let parsed = match evaluate_expressions(arguments, env.clone()) {
        Ok(all) => all,
        Err(v) => return v,
    };
    error_at(
        format!(
            "Call expression evaluation not implemented: {}",
            token.to_string()
        )
        .as_str(),
        token,
    )
}

fn evaluate_expressions(
    expressions: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Object>, Object> {
    let mut evaluated = Vec::with_capacity(expressions.len());
    for expression in expressions {
        let value = evaluate_expression(expression, env.clone());
        if let Object::Error { .. } = value {
            return Err(value);
        }
        if let Object::ReturnValue(_) = value {
            return Err(value);
        }
        evaluated.push(value);
    }
    Ok(evaluated)
}
