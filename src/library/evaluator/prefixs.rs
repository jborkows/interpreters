use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::{Expression, PrefixOperatorType},
    control_flow_dependent,
    object::{Environment, Object, error_at, type_of},
    tokens::Token,
};

use super::{FALSE, TRUE, evaluate_expression, int_value};

pub(super) fn prefix_operator_evaluation(
    token: &Token,
    operator: &PrefixOperatorType,
    as_ref: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let right = evaluate_expression(as_ref, env.clone());
    match operator {
        PrefixOperatorType::Bang => {
            control_flow_dependent!(right, bang_operator_evaluation(token, right.as_ref()));
        }
        PrefixOperatorType::Minus => {
            control_flow_dependent!(right, minus_operator_evaluation(token, right.as_ref()));
        }
    }
}

fn minus_operator_evaluation(token: &Token, right: &Object) -> Rc<Object> {
    match right {
        Object::Int(value) => {
            return Rc::new(int_value(-value));
        }
        _ => error_at(
            format!(
                "Minus (-) cannot be applied to {} ({})",
                type_of(&right),
                right.to_string()
            )
            .as_str(),
            token,
        ),
    }
}

fn bang_operator_evaluation(token: &Token, right: &Object) -> Rc<Object> {
    match right {
        Object::Boolean(value) => Rc::new(if *value { FALSE } else { TRUE }),
        Object::String(value) => Rc::new(if value.trim().is_empty() { TRUE } else { FALSE }),
        Object::Int(_value) => {
            return Rc::new(FALSE);
        }
        Object::Null => Rc::new(TRUE),
        _ => error_at(
            format!(
                "Bang operator cannot be used to {} ({})",
                type_of(&right),
                right.to_string()
            )
            .as_str(),
            token,
        ),
    }
}
