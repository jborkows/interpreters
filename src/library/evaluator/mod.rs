use infixs::infix_operator_evaluation;
use pool::*;
use prefixs::prefix_operator_evaluation;

use crate::{
    allocation_counting,
    ast::{
        base::Node,
        expression::Expression,
        statements::{Program, Statement},
    },
    object::Object,
    tokens::TokenKind,
};

#[cfg(test)]
mod evaluator_tests;
mod infixs;
mod object_pool;
mod pool;
mod prefixs;

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
