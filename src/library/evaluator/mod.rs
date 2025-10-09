use std::{cell::RefCell, rc::Rc};

use crate::ast::modify;
use crate::lines::TokenPosition;
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

pub fn expand_macros(program: Program, env: Rc<RefCell<Environment>>) -> Program {
    let modified = modify(Rc::new(program), |node| {
        let maybe_expression = node.as_any().downcast_ref::<Expression>();
        let expression = match maybe_expression {
            Some(v) => v,
            None => return node,
        };
        match expression {
            Expression::Call {
                token,
                function,
                arguments,
            } => {
                let obj = match is_macro_call(function, env.clone()) {
                    Some(v) => v,
                    None => return node,
                };

                return match obj.as_ref() {
                    Object::Macro {
                        parameters,
                        body,
                        env,
                    } => {
                        let mut extended_env = Environment::enclosed(env.clone());
                        let arguments = quote_arguments(arguments);
                        for (i, parameter) in parameters.iter().enumerate() {
                            extended_env
                                .set(parameter.name.clone(), arguments.get(i).unwrap().clone());
                        }
                        let evaluated =
                            evaluate(body.as_ref(), Rc::new(RefCell::new(extended_env)));
                        match evaluated.as_ref() {
                            Object::Quote(expression) => expression.clone(),
                            _ => panic!(
                                "You can only return AST node from macro. Returned {:?}",
                                evaluated
                            ),
                        }
                    }
                    _ => return node,
                };
            }
            _ => return node,
        }
    });
    let modified_program = modified
        .as_any()
        .downcast_ref::<Program>()
        .unwrap()
        .statements
        .clone();
    return Program {
        statements: modified_program,
    };
}

fn quote_arguments(arguments: &Vec<Expression>) -> Vec<Rc<Object>> {
    return arguments
        .iter()
        .map(|expression| Object::Quote(Rc::new(expression.clone())))
        .map(|obj| Rc::new(obj))
        .collect::<Vec<_>>();
}

fn is_macro_call(function: &Expression, env: Rc<RefCell<Environment>>) -> Option<Rc<Object>> {
    let identifier = match function {
        Expression::Identifier(token) => match &token.kind {
            TokenKind::Identifier(v) => v,
            _ => return None,
        },
        _ => return None,
    };

    return match env.borrow().get(&identifier) {
        Some(obj) => match obj.as_ref() {
            Object::Macro {
                parameters: _,
                body: _,
                env: _,
            } => Some(obj),
            _ => None,
        },
        None => return None,
    };
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
