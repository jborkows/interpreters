use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    object::{Environment, Object},
};

use super::evaluate_expression;
pub(super) fn evaluate_expressions(
    expressions: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Rc<Object>>, Rc<Object>> {
    let mut evaluated = Vec::with_capacity(expressions.len());
    for expression in expressions {
        let value = evaluate_expression(expression, env.clone());
        if let Object::Error { .. } = *value {
            return Err(value);
        }
        if let Object::ReturnValue(_) = *value {
            return Err(value);
        }
        evaluated.push(value);
    }
    Ok(evaluated)
}
