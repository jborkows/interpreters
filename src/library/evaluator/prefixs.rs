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
) -> Object {
    let right = evaluate_expression(as_ref, env.clone());
    match operator {
        PrefixOperatorType::Bang => {
            control_flow_dependent!(right, bang_operator_evaluation(token, right));
        }
        PrefixOperatorType::Minus => {
            control_flow_dependent!(right, minus_operator_evaluation(token, right));
        }
    }
}

fn minus_operator_evaluation(token: &Token, right: Object) -> Object {
    match right {
        Object::Int(value) => {
            return int_value(-value);
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

fn bang_operator_evaluation(token: &Token, right: Object) -> Object {
    match right {
        Object::Boolean(value) => {
            if value {
                return FALSE;
            } else {
                return TRUE;
            }
        }
        Object::String(value) => {
            if value.trim().is_empty() {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        Object::Int(_value) => {
            return FALSE;
        }
        Object::Null => {
            return TRUE;
        }
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
