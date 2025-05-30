use evaluator_expression::evaluate_expression;
use pool::*;

use crate::{
    ast::{
        base::Node,
        expression::Expression,
        statements::{Program, Statement},
    },
    object::Object,
};

mod evaluator_expression;
#[cfg(test)]
mod evaluator_tests;
mod infixs;
#[cfg(test)]
mod infixs_tests;
#[cfg(test)]
mod literals_tests;
mod object_pool;
mod pool;
mod prefixs;
#[cfg(test)]
mod prefixs_tests;

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
