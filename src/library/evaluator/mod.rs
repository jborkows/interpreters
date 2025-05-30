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
    object::Object,
};

mod evaluator_expression;
#[cfg(test)]
mod evaluator_tests;
mod if_expression;
#[cfg(test)]
mod if_expression_tests;
mod infixs;
#[cfg(test)]
mod infixs_tests;
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

fn evaluate_program(program: &Program) -> Object {
    let mut result = NULL;
    for statement in &program.statements {
        result = evaluate(statement);
        if let Object::ReturnValue(value) = result {
            return value.as_ref().clone();
        }
        if let Object::Error { .. } = result {
            return result;
        }
    }
    result
}

fn evaluate_block_statements(statements: &Vec<Statement>) -> Object {
    let mut result = NULL;
    for statement in statements {
        result = evaluate(statement);
        end_flow!(result);
    }
    result
}

fn evaluate_statement(statement: &Statement) -> Object {
    match statement {
        Statement::ExpressionStatement { expression, .. } => evaluate_expression(expression),
        Statement::BlockStatement {
            token: _,
            statements,
        } => evaluate_block_statements(statements),
        Statement::Return {
            token: _,
            return_value,
        } => {
            let return_value = evaluate_expression(return_value);
            return Object::ReturnValue(Rc::new(return_value));
        }
        _ => panic!(
            "Statement type not implemented: {:?}",
            statement.to_string()
        ),
    }
}
