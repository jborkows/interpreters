use crate::{
    object::{Environment, Object, error_at},
    tokens::{Token, TokenKind},
};

pub(super) fn evaluate_indentifier(token: &Token, env: &mut Environment) -> Object {
    match &token.kind {
        TokenKind::Identifier(name) => {
            if let Some(value) = env.get(name) {
                return value.clone();
            } else {
                return error_at(format!("Identifier '{}' not found.", name).as_str(), token);
            }
        }
        _ => error_at(
            format!(
                "Identifier evaluation not implemented: {}",
                token.to_string()
            )
            .as_str(),
            token,
        ),
    }
}
