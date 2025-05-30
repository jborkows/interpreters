use std::rc::Rc;

use evaluator_expression::evaluate_expression;
use pool::*;

use crate::{
    ast::{
        base::Node,
        expression::Expression,
        statements::{Program, Statement},
    },
    end_flow,
    object::{Environment, Object, error_at},
    tokens::{Token, TokenKind},
};

mod evaluate_identifier;
mod evaluator_expression;
#[cfg(test)]
mod evaluator_tests;
mod functional_literal_evaluations;
#[cfg(test)]
mod functions_tests;
#[cfg(test)]
mod if_expression_tests;
mod infixs;
#[cfg(test)]
mod infixs_tests;
#[cfg(test)]
mod let_tests;
#[cfg(test)]
mod literals_tests;
mod macros;
mod object_pool;
mod pool;
mod prefixs;
#[cfg(test)]
mod prefixs_tests;
#[cfg(test)]
mod return_tests;

pub fn evaluate(node: &dyn Node, env: &mut Environment) -> Object {
    let statement = node.as_any().downcast_ref::<Statement>();
    if let Some(statement) = statement {
        return evaluate_statement(statement, env);
    }
    let program = node.as_any().downcast_ref::<Program>();
    if let Some(program) = program {
        return evaluate_program(program, env);
    }

    let expression = node.as_any().downcast_ref::<Expression>();
    if let Some(expression) = expression {
        return evaluate_expression(expression, env);
    }
    panic!("Should never reach here, node: {:?}", node);
}

fn evaluate_program(program: &Program, env: &mut Environment) -> Object {
    let mut result = NULL;
    for statement in &program.statements {
        result = evaluate(statement, env);
        if let Object::ReturnValue(value) = result {
            return value.as_ref().clone();
        }
        if let Object::Error { .. } = result {
            return result;
        }
    }
    result
}

fn evaluate_block_statements(statements: &Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = NULL;
    for statement in statements {
        result = evaluate(statement, env);
        end_flow!(result);
    }
    result
}

fn evaluate_statement(statement: &Statement, env: &mut Environment) -> Object {
    match statement {
        Statement::ExpressionStatement { expression, .. } => evaluate_expression(expression, env),
        Statement::BlockStatement {
            token: _,
            statements,
        } => evaluate_block_statements(statements, env),
        Statement::Return {
            token: _,
            return_value,
        } => {
            let return_value = evaluate_expression(return_value, env);
            return Object::ReturnValue(Rc::new(return_value));
        }
        Statement::Let { token, name, value } => let_statement(token, name, value, env),
    }
}

fn let_statement(
    token: &Token,
    name: &Expression,
    value: &Expression,
    env: &mut Environment,
) -> Object {
    let name = match name {
        Expression::Identifier(token) => match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return error_at("Let statement name must be an identifier", token),
        },
        _ => return error_at("Let statement name must be an identifier", token),
    };
    let value = evaluate_expression(value, env);
    end_flow!(value);
    env.set(name, value.clone());
    value
}
