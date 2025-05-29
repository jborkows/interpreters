use pool::*;

use crate::{
    allocation_counting,
    ast::{
        base::Node,
        expression::{Expression, PrefixOperatorType},
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
        _ => panic!("Expression type not implemented: {:?}", expression),
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
