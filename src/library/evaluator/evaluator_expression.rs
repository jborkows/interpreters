use std::{cell::RefCell, rc::Rc};

use crate::{
    allocation_counting,
    ast::expression::Expression,
    end_flow,
    object::{Environment, Object},
    tokens::TokenKind,
};

use super::{
    arrays::{parse_array_literal, parse_index_expression},
    evaluate,
    evaluate_call::evaluate_call_expression,
    evaluate_identifier::evaluate_indentifier,
    functional_literal_evaluations::function_literal_evaluation,
    infixs::infix_operator_evaluation,
    int_value,
    prefixs::prefix_operator_evaluation,
    string_value,
};
use crate::object::*;

pub(super) fn evaluate_expression(
    expression: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    match expression {
        Expression::IntegerLiteral(token) => {
            match token.as_ref().kind {
                TokenKind::Integer(value) => {
                    // Handle integer literal evaluation
                    let value = value as i64;
                    allocation_counting!(int_value(value), value)
                }
                _ => unreachable!("Expected an integer token, got: {:?}", token),
            }
        }
        Expression::BooleanLiteral { token, value: _ } => match token.as_ref().kind {
            TokenKind::True => true_value(),
            TokenKind::False => false_value(),
            _ => unreachable!("Expected a boolean token, got: {:?}", token),
        },
        Expression::StringLiteral(token) => match token.as_ref().kind {
            TokenKind::StringLiteral(ref value) => string_value(value.to_string()),
            _ => unreachable!("Expected a string token, got: {:?}", token),
        },
        Expression::PrefixOperator {
            token,
            operator,
            right,
        } => prefix_operator_evaluation(token, operator, right.as_ref(), env.clone()),
        Expression::Infix {
            token,
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(left, env.clone());
            end_flow!(left_value);
            let right_value = evaluate_expression(right, env.clone());
            end_flow!(right_value);
            infix_operator_evaluation(token, operator, left_value, right_value)
        }
        Expression::AIf {
            token: _,
            condition,
            consequence,
            alternative,
        } => {
            let condition_value = evaluate_expression(condition, env.clone());
            if is_truthy(condition_value.as_ref()) {
                evaluate(consequence.as_ref(), env.clone())
            } else if let Some(alternative) = alternative {
                evaluate(alternative.as_ref(), env.clone())
            } else {
                null_value()
            }
        }
        Expression::Identifier(token) => evaluate_indentifier(token, env.clone()),
        Expression::Call {
            token,
            function,
            arguments,
        } => evaluate_call_expression(token, function, arguments, env.clone()),
        Expression::FunctionLiteral {
            token,
            parameters,
            body,
        } => function_literal_evaluation(token, parameters, body, env.clone()),
        Expression::ArrayLiteral { token, elements } => parse_array_literal(elements, env.clone()),
        Expression::Index {
            token,
            array,
            index,
        } => parse_index_expression(token, array, index, env.clone()),
        //TODO: implement MapLiteral
        Expression::MapLiteral { token, elements } => todo!(),
    }
}
