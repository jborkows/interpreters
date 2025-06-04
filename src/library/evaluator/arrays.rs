use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    end_flow,
    evaluator::evaluate_expressions::evaluate_expressions,
    object::{Environment, Object, error_at},
    tokens::Token,
};

use super::evaluate_expression;

pub(super) fn parse_array_literal(
    elements: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let parsed = match evaluate_expressions(elements, env.clone()) {
        Ok(all) => all,
        Err(v) => return v,
    };
    return Rc::new(Object::Array { elements: parsed });
}

pub(super) fn parse_index_expression(
    token: &Token,
    array: &Expression,
    index: &Expression,
    clone: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let left_value = evaluate_expression(array, clone.clone());
    end_flow!(left_value);
    let right_value = evaluate_expression(index, clone.clone());
    end_flow!(right_value);
    return evaluate_index_expression(token, left_value, right_value);
}

fn evaluate_index_expression(
    token: &Token,
    left_value: Rc<Object>,
    right_value: Rc<Object>,
) -> Rc<Object> {
    match *left_value {
        Object::Array { ref elements } => {
            if let Object::Int(value) = *right_value {
                return parse_integer_index(token, elements, value);
            }
            error_at("Index must be an integer", token)
        }
        _ => error_at("Index operator can only be applied to arrays", token),
    }
}

fn parse_integer_index(token: &Token, elements: &[Rc<Object>], value: i64) -> Rc<Object> {
    if value == 0 {
        error_at(
            format!("Index out of bounds: {} (arrays are 1 indexed)", value).as_str(),
            token,
        )
    } else if (value.abs() as usize) > elements.len() {
        error_at(format!("Index out of bounds: {}", value).as_str(), token)
    } else if value < 0 {
        elements[(elements.len() as i64 + value) as usize].clone()
    } else {
        elements[value as usize - 1].clone()
    }
}
