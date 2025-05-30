use crate::{
    allocation_counting,
    ast::expression::Expression,
    end_flow,
    object::{Environment, Object, error_at},
    tokens::TokenKind,
};

use super::{
    FALSE, NULL, TRUE, evaluate, evaluate_identifier::evaluate_indentifier,
    functional_literal_evaluations::function_literal_evaluation, infixs::infix_operator_evaluation,
    int_value, prefixs::prefix_operator_evaluation, string_value,
};

pub(super) fn evaluate_expression(expression: &Expression, env: &mut Environment) -> Object {
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
        } => prefix_operator_evaluation(token, operator, right.as_ref(), env),
        Expression::InfixExpression {
            token,
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(left, env);
            end_flow!(left_value);
            let right_value = evaluate_expression(right, env);
            end_flow!(right_value);
            return infix_operator_evaluation(token, operator, left_value, right_value);
        }
        Expression::IfExpression {
            token: _,
            condition,
            consequence,
            alternative,
        } => {
            let condition_value = evaluate_expression(condition, env);
            if is_truthy(condition_value) {
                return evaluate(consequence.as_ref(), env);
            } else if let Some(alternative) = alternative {
                return evaluate(alternative.as_ref(), env);
            } else {
                return NULL;
            }
        }
        Expression::Identifier(token) => evaluate_indentifier(token, env),
        Expression::CallExpression {
            token,
            function,
            arguments,
        } => error_at(
            format!(
                "Call expression evaluation not implemented: {}",
                token.to_string()
            )
            .as_str(),
            token,
        ),
        Expression::FunctionLiteral {
            token,
            parameters,
            body,
        } => function_literal_evaluation(token, parameters, body, env),
    }
}

fn is_truthy(condition_value: Object) -> bool {
    if condition_value == NULL {
        return false;
    }
    if condition_value == FALSE {
        return false;
    }
    return true;
}
