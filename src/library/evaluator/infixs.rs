use crate::{ast::expression::InfixOperatorType, object::Object, tokens::Token};

use super::{FALSE, TRUE, int_value, string_value};
pub(super) fn infix_operator_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left: Object,
    right: Object,
) -> Object {
    let some_value: Option<Object> = match left {
        Object::Int(left_value) => match right {
            Object::Int(right_value) => Some(integer_only_infix_operator_evaluation(
                token,
                operator,
                left_value,
                right_value,
            )),
            Object::String(ref right_value) => Some(int_to_string_infix_evaluation(
                token,
                operator,
                left_value,
                right_value,
            )),
            _ => None,
        },
        Object::String(ref left_value) => match right {
            Object::Int(right_value) => Some(string_to_int_infix_evaluation(
                token,
                operator,
                left_value,
                right_value,
            )),
            Object::String(ref right_value) => Some(string_infix_evaluation(
                token,
                operator,
                left_value,
                right_value,
            )),
            _ => None,
        },
        Object::Boolean(_) => None,
        Object::Null => None,
    };
    return some_value.unwrap_or_else(|| {
        panic!(
            "Infix operator evaluation failed for token: {} with left value: {} and right value: {}",
            token.to_string(),
            left.to_string(),
            right.to_string()
        )
    });
}

fn string_infix_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left: &String,
    right: &String,
) -> Object {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left, right);
            return string_value(result);
        }
        InfixOperatorType::Multiply => {
            let mut result = String::new();
            for _ in 0..right.parse::<usize>().unwrap_or(0) {
                result.push_str(left);
            }
            return string_value(result);
        }
        InfixOperatorType::Equal => {
            if left == right {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        InfixOperatorType::NotEqual => {
            if left != right {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        _ => panic!(
            "Infix operator evaluation not implemented for strings: {:?} for token: {}",
            operator,
            token.to_string()
        ),
    }
}
fn int_to_string_infix_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left_value: i64,
    right_value: &str,
) -> Object {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left_value, right_value);
            return string_value(result);
        }
        _ => panic!(
            "Infix operator evaluation not implemented for string and integer: {:?} for token: {}",
            operator,
            token.to_string()
        ),
    }
}

fn string_to_int_infix_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left: &String,
    right_value: i64,
) -> Object {
    match operator {
        InfixOperatorType::Plus => {
            let result = format!("{}{}", left, right_value);
            return string_value(result);
        }
        InfixOperatorType::Multiply => {
            let mut result = String::new();
            for _ in 0..right_value {
                result.push_str(left);
            }
            return string_value(result);
        }
        _ => panic!(
            "Infix operator evaluation not implemented for string and integer: {:?} for token: {}",
            operator,
            token.to_string()
        ),
    }
}

fn integer_only_infix_operator_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left_value: i64,
    right_value: i64,
) -> Object {
    match operator {
        InfixOperatorType::Plus => int_value(left_value + right_value),
        InfixOperatorType::Minus => int_value(left_value - right_value),
        InfixOperatorType::Multiply => int_value(left_value * right_value),
        InfixOperatorType::Divide => {
            if right_value == 0 {
                panic!("Division by zero error at {}", token.at_text());
            }
            int_value(left_value / right_value)
        }
        InfixOperatorType::Equal => {
            if left_value == right_value {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        InfixOperatorType::NotEqual => {
            if left_value != right_value {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        InfixOperatorType::GreaterThan => {
            if left_value > right_value {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        InfixOperatorType::LessThan => {
            if left_value < right_value {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        _ => panic!(
            "Infix operator evaluation not implemented: {:?} for token: {}",
            operator,
            token.to_string()
        ),
    }
}
