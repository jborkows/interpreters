use std::{cell::RefCell, rc::Rc};

use crate::object::*;
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
use evaluator_expression::evaluate_expression;

mod arrays;
mod evaluate_call;
mod evaluate_expressions;
mod evaluate_identifier;
mod evaluator_expression;
mod functional_literal_evaluations;
mod infixs;
mod macros;
mod maps;
mod prefixs;
#[cfg(test)]
mod tests;

pub fn evaluate(node: &dyn Node, env: Rc<RefCell<Environment>>) -> Rc<Object> {
    let statement = node.as_any().downcast_ref::<Statement>();
    if let Some(statement) = statement {
        return evaluate_statement(statement, env.clone());
    }
    let program = node.as_any().downcast_ref::<Program>();
    if let Some(program) = program {
        return evaluate_program(program, env.clone());
    }

    let expression = node.as_any().downcast_ref::<Expression>();
    if let Some(expression) = expression {
        return evaluate_expression(expression, env.clone());
    }
    panic!("Should never reach here, node: {:?}", node);
}

fn evaluate_program(program: &Program, env: Rc<RefCell<Environment>>) -> Rc<Object> {
    let mut result = null_value();
    for statement in &program.statements {
        result = evaluate(statement, env.clone());
        if let Object::ReturnValue(value) = result.as_ref() {
            return value.clone();
        }
        if let Object::Error { .. } = result.as_ref() {
            return result;
        }
    }
    result
}

fn evaluate_block_statements(
    statements: &Vec<Statement>,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let mut result = null_value();
    for statement in statements {
        result = evaluate(statement, env.clone());
        end_flow!(result);
    }
    result
}

fn evaluate_statement(statement: &Statement, env: Rc<RefCell<Environment>>) -> Rc<Object> {
    match statement {
        Statement::AExpression { expression, .. } => evaluate_expression(expression, env.clone()),
        Statement::Block {
            token: _,
            statements,
        } => evaluate_block_statements(statements, env.clone()),
        Statement::Return {
            token: _,
            return_value,
        } => {
            let return_value = evaluate_expression(return_value, env.clone());
            Rc::new(Object::ReturnValue(return_value))
        }
        Statement::Let { token, name, value } => let_statement(token, name, value, env.clone()),
    }
}

fn let_statement(
    token: &Token,
    name: &Expression,
    value: &Expression,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let name = match name {
        Expression::Identifier(token) => match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return error_at("Let statement name must be an identifier", token),
        },
        _ => return error_at("Let statement name must be an identifier", token),
    };
    let value = evaluate_expression(value, env.clone());
    end_flow!(value);
    env.borrow_mut().set(name, value.clone());
    value
}

//TODO: make more then top level macros
pub fn define_macros(program: Program, env: Rc<RefCell<Environment>>) -> Program {
    let macros = program
        .statements
        .iter()
        .filter(|s| is_macro(s))
        .collect::<Vec<_>>();
    let rest = program
        .statements
        .iter()
        .filter(|s| !is_macro(s))
        .map(|f| f.to_owned())
        .collect::<Vec<_>>();
    macros.iter().for_each(|m| match m {
        Statement::Let {
            token: _,
            name,
            value,
        } => match value {
            Expression::MacroLiteral {
                token: _,
                parameters,
                body,
            } => {
                println!("Processing {:?}", &m);
                let name_value = match name {
                    Expression::Identifier(token) => match &token.kind {
                        TokenKind::Identifier(v) => v.to_string(),
                        _ => panic!("It has to be Identifier but found {:?}", token.kind),
                    },
                    _ => panic!("It has to be Identifier but found {:?}", name),
                };

                let mut parsed_parameters: Vec<Identifier> = vec![];

                for parameter in parameters.as_ref() {
                    match parameter {
                        Expression::Identifier(id_token) => match &id_token.kind {
                            TokenKind::Identifier(value) => {
                                parsed_parameters.push(Identifier {
                                    name: value.clone(),
                                });
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                let a_macro = Object::Macro {
                    parameters: parsed_parameters,
                    body: Rc::new(body.as_ref().clone()),
                    env: env.clone(),
                };
                env.borrow_mut().set(name_value, Rc::new(a_macro));
            }
            _ => {}
        },
        _ => {}
    });
    return Program { statements: rest };
}

fn is_macro(statement: &Statement) -> bool {
    match statement {
        Statement::Let {
            token: _,
            name: _,
            value,
        } => match value {
            Expression::MacroLiteral {
                token: _,
                parameters: _,
                body: _,
            } => true,
            _ => false,
        },
        _ => false,
    }
}
