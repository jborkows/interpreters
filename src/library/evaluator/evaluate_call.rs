use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    end_flow,
    evaluator::{evaluate, evaluate_expressions::evaluate_expressions},
    object::{Environment, Identifier, Object, error_at},
    tokens::{Token, TokenKind},
};

use super::evaluate_expression;

pub fn evaluate_call_expression(
    token: &Token,
    function: &Expression,
    arguments: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    match function {
        Expression::Identifier(id) => match &id.kind {
            TokenKind::Identifier(name) => {
                if name == "quote" {
                    let first = &arguments[0];
                    return Rc::new(Object::Quote(Rc::new(first.clone())));
                }
            }
            _ => {}
        },
        _ => {}
    }
    let function = evaluate_expression(function, env.clone());
    end_flow!(function);

    let parsed = match evaluate_expressions(arguments, env.clone()) {
        Ok(all) => all,
        Err(v) => return v,
    };
    if parsed.len() != arguments.len() {
        return error_at(
            format!(
                "Function call expected {} arguments, got {}",
                arguments.len(),
                parsed.len()
            )
            .as_str(),
            token,
        );
    }
    apply_function(token, function, &parsed)
}

fn apply_function(token: &Token, function: Rc<Object>, arguments: &[Rc<Object>]) -> Rc<Object> {
    match *function {
        Object::Function {
            ref parameters,
            ref body,
            env: ref func_env,
        } => {
            /*
             * Function environment is extended so even if variables are not visible in current scope they can
             * still be accessed in the function body.
             */
            let extended_env = extend_env(func_env.clone(), parameters, arguments);
            let body_fun = body.as_ref();
            let result = evaluate(body_fun, extended_env);
            match *result {
                Object::ReturnValue(ref value) => value.clone(),
                Object::Error { .. } => result,
                _ => result,
            }
        }
        Object::Builtin(ref func) => func.apply(token, arguments),
        _ => error_at("Call expression is not a function.", token),
    }
}

fn extend_env(
    clone: Rc<RefCell<Environment>>,
    parameters: &[Identifier],
    arguments: &[Rc<Object>],
) -> Rc<RefCell<Environment>> {
    let new_env = Rc::new(RefCell::new(Environment::enclosed(clone)));
    parameters
        .iter()
        .zip(arguments.iter())
        .for_each(|(param, arg)| new_env.borrow_mut().set(param.name.clone(), arg.clone()));
    new_env
}
