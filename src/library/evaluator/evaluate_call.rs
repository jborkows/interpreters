use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    end_flow,
    evaluator::evaluate,
    object::{Environment, Object, error_at},
    tokens::Token,
};

use super::evaluate_expression;

pub fn evaluate_call_expression(
    token: &Token,
    function: &Expression,
    arguments: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
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
    return apply_function(token, function, &parsed, env);
}

fn apply_function(
    token: &Token,
    function: Rc<Object>,
    parsed: &[Rc<Object>],
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    match *function {
        Object::Function {
            ref parameters,
            ref body,
            env: ref func_env,
        } => {
            let extended_env = Rc::new(RefCell::new(Environment::enclosed(func_env.clone())));
            let body_fun = body.as_ref();
            let result = evaluate(body_fun, extended_env);
            end_flow!(result);
            return result;
        }
        _ => return error_at("Call expression is not a function.", token),
    }
    todo!()
}

fn evaluate_expressions(
    expressions: &[Expression],
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Rc<Object>>, Rc<Object>> {
    let mut evaluated = Vec::with_capacity(expressions.len());
    for expression in expressions {
        let value = evaluate_expression(expression, env.clone());
        if let Object::Error { .. } = *value {
            return Err(value);
        }
        if let Object::ReturnValue(_) = *value {
            return Err(value);
        }
        evaluated.push(value);
    }
    Ok(evaluated)
}
