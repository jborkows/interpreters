use std::{cell::RefCell, rc::Rc};

use crate::{
    object::{Environment, Object, error_at, parse_built_in_function},
    tokens::{Token, TokenKind},
};

pub(super) fn evaluate_indentifier(token: &Token, env: Rc<RefCell<Environment>>) -> Rc<Object> {
    match &token.kind {
        TokenKind::Identifier(name) => {
            if let Some(value) = env.borrow().get(name) {
                return value.clone();
            }
            if let Some(value) = parse_built_in_function(name) {
                return Rc::new(Object::Builtin(value));
            }
            error_at(format!("Identifier '{}' not found.", name).as_str(), token)
        }
        _ => error_at(
            format!("Identifier evaluation not implemented: {}", token).as_str(),
            token,
        ),
    }
}
