use std::{cell::RefCell, rc::Rc};

use crate::{
    allocation_counting,
    ast::expression::Expression,
    end_flow,
    object::{Environment, Object, error_at},
    tokens::TokenKind,
};

use super::{
    FALSE, NULL, TRUE, evaluate, evaluate_call::evaluate_call_expression,
    evaluate_identifier::evaluate_indentifier,
    functional_literal_evaluations::function_literal_evaluation, infixs::infix_operator_evaluation,
    int_value, prefixs::prefix_operator_evaluation, string_value,
};

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
                    return Rc::new(allocation_counting!(int_value(value), value));
                }
                _ => unreachable!("Expected an integer token, got: {:?}", token),
            }
        }
        Expression::BooleanLiteral { token, value: _ } => match token.as_ref().kind {
            TokenKind::True => {
                return Rc::new(TRUE);
            }
            TokenKind::False => {
                return Rc::new(FALSE);
            }
            _ => unreachable!("Expected a boolean token, got: {:?}", token),
        },
        Expression::StringLiteral(token) => match token.as_ref().kind {
            TokenKind::StringLiteral(ref value) => {
                return Rc::new(string_value(value.to_string()));
            }
            _ => unreachable!("Expected a string token, got: {:?}", token),
        },
        Expression::PrefixOperator {
            token,
            operator,
            right,
        } => prefix_operator_evaluation(token, operator, right.as_ref(), env.clone()),
        Expression::InfixExpression {
            token,
            left,
            operator,
            right,
        } => {
            let left_value = evaluate_expression(left, env.clone());
            end_flow!(left_value);
            let right_value = evaluate_expression(right, env.clone());
            end_flow!(right_value);
            return infix_operator_evaluation(token, operator, left_value, right_value);
        }
        Expression::IfExpression {
            token: _,
            condition,
            consequence,
            alternative,
        } => {
            let condition_value = evaluate_expression(condition, env.clone());
            if is_truthy(condition_value.as_ref()) {
                return evaluate(consequence.as_ref(), env.clone());
            } else if let Some(alternative) = alternative {
                return evaluate(alternative.as_ref(), env.clone());
            } else {
                return Rc::new(NULL);
            }
        }
        Expression::Identifier(token) => evaluate_indentifier(token, env.clone()),
        Expression::CallExpression {
            token,
            function,
            arguments,
        } => evaluate_call_expression(token, function, arguments, env.clone()),
        Expression::FunctionLiteral {
            token,
            parameters,
            body,
        } => function_literal_evaluation(token, parameters, body, env.clone()),
    }
}

fn is_truthy(condition_value: &Object) -> bool {
    if *condition_value == NULL {
        return false;
    }
    if *condition_value == FALSE {
        return false;
    }
    return true;
}
