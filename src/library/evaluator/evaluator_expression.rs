use crate::{allocation_counting, ast::expression::Expression, object::Object, tokens::TokenKind};

use super::{
    FALSE, TRUE, infixs::infix_operator_evaluation, int_value, prefixs::prefix_operator_evaluation,
    string_value,
};

pub(super) fn evaluate_expression(expression: &Expression) -> Object {
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
