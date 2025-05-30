use std::rc::Rc;

use crate::tokens::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    String(String),
    Boolean(bool),
    ReturnValue(Rc<Object>),
    Error {
        message: String,
        line: usize,
        column: usize,
    },
    Null,
}

pub fn error_at(message: &str, token: &Token) -> Object {
    let position = token.position();
    Object::Error {
        message: message.to_string(),
        line: position.0,
        column: position.1,
    }
}

pub fn type_of(object: &Object) -> String {
    match object {
        Object::Int(_) => "Int".to_string(),
        Object::String(_) => "String".to_string(),
        Object::Boolean(_) => "Boolean".to_string(),
        Object::ReturnValue(_) => "ReturnValue".to_string(),
        Object::Error { .. } => "Error".to_string(),
        Object::Null => "Null".to_string(),
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::String(s) => s.clone(),
            Object::Boolean(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Object::Null => "NULL".to_string(),
            Object::ReturnValue(object) => object.to_string(),
            Object::Error {
                message,
                line,
                column,
            } => {
                format!("Error at {}:{} -> {}", line, column, message)
            }
        }
    }
}
