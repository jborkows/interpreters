use crate::{
    ast::expression::{Expression, PrefixOperatorType},
    object::Object,
    tokens::Token,
};

use super::{FALSE, TRUE, evaluate_expression, int_value};

pub(super) fn prefix_operator_evaluation(
    token: &Token,
    operator: &PrefixOperatorType,
    as_ref: &Expression,
) -> Object {
    match operator {
        PrefixOperatorType::Bang => {
            let right = evaluate_expression(as_ref);
            if let Object::ReturnValue(_) = right {
                return right;
            }
            return bang_operator_evaluation(token, right);
        }
        PrefixOperatorType::Minus => {
            let right = evaluate_expression(as_ref);
            if let Object::ReturnValue(_) = right {
                return right;
            }
            return minus_operator_evaluation(token, right);
        }
    }
}

fn minus_operator_evaluation(token: &Token, right: Object) -> Object {
    match right {
        Object::Int(value) => {
            return int_value(-value);
        }
        _ => panic!(
            "Minus operator can only be applied to integer values. Error at {}",
            token.at_text()
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
        _ => panic!(
            "Bang operator can only be applied to boolean, string, or integer values: at {}",
            token.at_text()
        ),
    }
}
