use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::Node,
        expression::{self, Expression},
        modify,
    },
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
                    return evaluate_quote(&arguments[0], &token, env.clone());
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

fn evaluate_quote(
    argument: &Expression,
    token: &Token,
    env: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let modified_argument = evaluate_unqote(argument.clone(), &token, env);
    return Rc::new(Object::Quote(modified_argument));
}

fn evaluate_unqote(
    expression: Expression,
    token: &Token,
    env: Rc<RefCell<Environment>>,
) -> Rc<Expression> {
    fn traverse<'a>(
        node: Rc<dyn Node + 'a>,
        token: &Token,
        env: Rc<RefCell<Environment>>,
    ) -> Rc<dyn Node + 'a> {
        let expression = node.as_any().downcast_ref::<Expression>();
        let expression = match expression {
            Some(v) => v,
            None => return node,
        };
        if !is_unquote_call(expression) {
            return node;
        }

        return match expression {
            Expression::Call {
                token: _,
                function: _,
                arguments,
            } => {
                if arguments.len() != 1 {
                    return node;
                }

                println!("unquoting {:?}", &arguments[0]);
                let unqoted = evaluate(&arguments[0], env);
                return convert_unqoted_into_ast(unqoted, token);
            }
            _ => node,
        };
    }
    let node = modify(Rc::new(expression), |n| traverse(n, token, env.clone()))
        .as_any()
        .downcast_ref::<Expression>()
        .unwrap()
        .clone();
    Rc::new(node)
}

fn convert_unqoted_into_ast(unqoted: Rc<Object>, token: &Token) -> Rc<Expression> {
    let position = token.position();
    let position =
        crate::lines::TokenPosition::single_character(position.0.into(), position.1.into());
    let expression = match *unqoted {
        Object::Int(v) => {
            if v > 0 {
                Expression::IntegerLiteral(Rc::new(Token::new(
                    position,
                    TokenKind::Integer(v.try_into().unwrap()),
                )))
            } else {
                todo!("minus case")
            }
        }
        _ => todo!("To fill"),
    };
    Rc::new(expression)
}

fn is_unquote_call(expression: &Expression) -> bool {
    match expression {
        Expression::Call {
            token: _,
            function,
            arguments: _,
        } => {
            if let Expression::Identifier(token) = *function.clone() {
                match &token.kind {
                    TokenKind::Identifier(function_name) => function_name == "unquote",
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
