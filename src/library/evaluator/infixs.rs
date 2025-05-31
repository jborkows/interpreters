use std::rc::Rc;

use crate::object::*;
use crate::{
    ast::expression::InfixOperatorType,
    object::{Object, error_at, type_of},
    tokens::Token,
};

pub(super) fn infix_operator_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left: Rc<Object>,
    right: Rc<Object>,
) -> Rc<Object> {
    let some_value: Option<Rc<Object>> = match *left {
        Object::Int(left_value) => match *right {
            Object::Int(right_value) => {
                integer_only_infix_operator_evaluation(token, operator, left_value, right_value)
            }
            Object::String(ref right_value) => {
                int_to_string_infix_evaluation(operator, left_value, right_value)
            }
            _ => None,
        },
        Object::String(ref left_value) => match *right {
            Object::Int(right_value) => {
                string_to_int_infix_evaluation(operator, left_value, right_value)
            }
            Object::String(ref right_value) => {
                string_infix_evaluation(operator, left_value, right_value)
            }
            Object::Boolean(_) | Object::Null => {
                string_infix_evaluation(operator, left_value, &right.to_string())
            }
            _ => None,
        },
        Object::Boolean(left_value) => match *right {
            Object::Boolean(right_value) => match operator {
                InfixOperatorType::Equal => Some(if left_value == right_value {
                    true_value()
                } else {
                    false_value()
                }),
                InfixOperatorType::NotEqual => Some(if left_value != right_value {
                    true_value()
                } else {
                    false_value()
                }),
                _ => None,
            },
            Object::Int(right_value) => int_to_string_infix_evaluation(
                operator,
                left_value as i64,
                &right_value.to_string(),
            ),
            Object::String(ref right_value) => {
                string_infix_evaluation(operator, &left_value.to_string(), &right_value)
            }
            _ => None,
        },
        _ => None,
    };
    return some_value.unwrap_or_else(|| {
        error_at(
            format!(
                "Cannot use {} on {}({}) and {}({})",
                operator.to_string(),
                type_of(&left),
                left.to_string(),
                type_of(&right),
                right.to_string()
            )
            .as_str(),
            token,
        )
    });
}

fn string_infix_evaluation(
    operator: &InfixOperatorType,
    left: &String,
    right: &String,
) -> Option<Rc<Object>> {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left, right);
            return Some(string_value(result));
        }
        InfixOperatorType::Equal => Some(boolean_value(left == right)),
        InfixOperatorType::NotEqual => Some(boolean_value(left != right)),
        _ => None,
    }
}
fn int_to_string_infix_evaluation(
    operator: &InfixOperatorType,
    left_value: i64,
    right_value: &str,
) -> Option<Rc<Object>> {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left_value, right_value);
            return Some(string_value(result));
        }
        _ => None,
    }
}

fn string_to_int_infix_evaluation(
    operator: &InfixOperatorType,
    left: &String,
    right_value: i64,
) -> Option<Rc<Object>> {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left, right_value);
            return Some(string_value(result));
        }
        InfixOperatorType::Multiply => {
            let mut result = String::new();
            for _ in 0..right_value {
                result.push_str(left);
            }
            return Some(string_value(result));
        }
        _ => None,
    }
}

fn integer_only_infix_operator_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left_value: i64,
    right_value: i64,
) -> Option<Rc<Object>> {
    match operator {
        InfixOperatorType::Plus => Some(int_value(left_value + right_value)),
        InfixOperatorType::Minus => Some(int_value(left_value - right_value)),
        InfixOperatorType::Multiply => Some(int_value(left_value * right_value)),
        InfixOperatorType::Divide => {
            if right_value == 0 {
                Some(error_at("Division by zero is not allowed", token))
            } else {
                Some(int_value(left_value / right_value))
            }
        }
        InfixOperatorType::Equal => Some(boolean_value(left_value == right_value)),
        InfixOperatorType::NotEqual => Some(boolean_value(left_value != right_value)),
        InfixOperatorType::LessThan => Some(boolean_value(left_value < right_value)),
        InfixOperatorType::GreaterThan => Some(boolean_value(left_value > right_value)),
    }
}
