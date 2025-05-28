use pool::*;

use crate::ast::{
    base::Node,
    expression::Expression,
    statements::{Program, Statement},
};

#[cfg(test)]
mod evaluator_tests;
mod pool;

pub fn evaluate(node: &dyn Node) -> crate::object::Object {
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

fn evaluate_expression(expression: &Expression) -> crate::object::Object {
    match expression {
        Expression::IntegerLiteral(token) => {
            match token.as_ref().kind {
                crate::tokens::TokenKind::Integer(value) => {
                    // Handle integer literal evaluation
                    return crate::object::Object::Int(value.into());
                }
                _ => unreachable!("Expected an integer token, got: {:?}", token),
            }
        }
        Expression::BooleanLiteral { token, value: _ } => match token.as_ref().kind {
            crate::tokens::TokenKind::True => {
                return TRUE;
            }
            crate::tokens::TokenKind::False => {
                return FALSE;
            }
            _ => unreachable!("Expected a boolean token, got: {:?}", token),
        },
        _ => panic!("Expression type not implemented: {:?}", expression),
    }
}

fn evaluate_program(program: &Program) -> crate::object::Object {
    let mut result = NULL;
    for statement in &program.statements {
        result = evaluate(statement);
    }
    result
}

fn evaluate_statement(statement: &Statement) -> crate::object::Object {
    match statement {
        Statement::ExpressionStatement { expression, .. } => evaluate_expression(expression),
        _ => panic!(
            "Statement type not implemented: {:?}",
            statement.to_string()
        ),
    }
}
