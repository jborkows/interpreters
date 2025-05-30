use pool::*;

use crate::{
    allocation_counting,
    ast::{
        base::Node,
        expression::{Expression, InfixOperatorType, PrefixOperatorType},
        statements::{Program, Statement},
    },
    object::Object,
    tokens::{Token, TokenKind},
};

#[cfg(test)]
mod evaluator_tests;
mod object_pool;
mod pool;

pub fn evaluate(node: &dyn Node) -> Object {
    let statement = node.as_any().downcast_ref::<Statement>();
    if let Some(statement) = statement {
        return evaluate_statement(statement);
    }
    let program = node.as_any().downcast_ref::<Program>();
    if let Some(program) = program {
        return evaluate_program(program);
    }

    let expression = node.as_any().downcast_ref::<Expression>();
    if let Some(expression) = expression {
        return evaluate_expression(expression);
    }
    panic!("Not implemented yet");
}

fn evaluate_expression(expression: &Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(token) => {
            match token.as_ref().kind {
                TokenKind::Integer(value) => {
                    // Handle integer literal evaluation
                    let value = value as i64;
                    return allocation_counting!(int_value(value), value);
                }
                _ => unreachable!("Expected an integer token, got: {:?}", token),
            }
        }
        Expression::BooleanLiteral { token, value: _ } => match token.as_ref().kind {
            TokenKind::True => {
                return TRUE;
            }
            TokenKind::False => {
                return FALSE;
            }
            _ => unreachable!("Expected a boolean token, got: {:?}", token),
        },
        Expression::StringLiteral(token) => match token.as_ref().kind {
            TokenKind::StringLiteral(ref value) => {
                return string_value(value.to_string());
            }
            _ => unreachable!("Expected a string token, got: {:?}", token),
        },
        Expression::PrefixOperator {
            token,
            operator,
            right,
        } => prefix_operator_evaluation(token, operator, right.as_ref()),
        Expression::InfixExpression {
            token,
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(left);
            let right_value = evaluate_expression(right);
            return infix_operator_evaluation(token, operator, left_value, right_value);
        }
        _ => panic!("Expression type not implemented: {:?}", expression),
    }
}

fn infix_operator_evaluation(
    token: &Token,
    operator: &InfixOperatorType,
    left: Object,
    right: Object,
) -> Object {
    let some_value: Option<Object> = match left {
        Object::Int(left_value) => match right {
            Object::Int(right_value) => Some(infix_operator_evaluation_int(
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

fn infix_operator_evaluation_int(
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
        _ => panic!(
            "Infix operator evaluation not implemented: {:?} for token: {}",
            operator,
            token.to_string()
        ),
    }
}

fn prefix_operator_evaluation(
    token: &Token,
    operator: &PrefixOperatorType,
    as_ref: &Expression,
) -> Object {
    match operator {
        PrefixOperatorType::Bang => {
            let right = evaluate_expression(as_ref);
            return bang_operator_evaluation(token, right);
        }
        PrefixOperatorType::Minus => {
            let right = evaluate_expression(as_ref);
            return minus_operator_evaluation(token, right);
        }
        _ => panic!(
            "Prefix operator evaluation not implemented: {:?} for token: {}",
            operator,
            token.to_string()
        ),
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

fn evaluate_program(program: &Program) -> Object {
    let mut result = NULL;
    for statement in &program.statements {
        result = evaluate(statement);
    }
    result
}

fn evaluate_statement(statement: &Statement) -> Object {
    match statement {
        Statement::ExpressionStatement { expression, .. } => evaluate_expression(expression),
        _ => panic!(
            "Statement type not implemented: {:?}",
            statement.to_string()
        ),
    }
}
