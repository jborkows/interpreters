use std::{cell::RefCell, rc::Rc};

use crate::{ast::statements::Statement, join_collection, tokens::Token};
mod builtins;
mod environment;
mod helpers;
mod object_pool;
use builtins::BuiltInFunction;
pub use builtins::parse_built_in_function;
pub use environment::Environment;
pub use helpers::*;

#[derive(Debug, Clone)]
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
    Function {
        parameters: Vec<Identifier>,
        body: Rc<Statement>,
        env: Rc<RefCell<Environment>>,
    },
    Builtin(BuiltInFunction),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::ReturnValue(l0), Self::ReturnValue(r0)) => l0 == r0,
            (
                Self::Error {
                    message: l_message,
                    line: l_line,
                    column: l_column,
                },
                Self::Error {
                    message: r_message,
                    line: r_line,
                    column: r_column,
                },
            ) => l_message == r_message && l_line == r_line && l_column == r_column,
            (Self::Function { .. }, Self::Function { .. }) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}
impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

pub fn error_at(message: &str, token: &Token) -> Rc<Object> {
    let position = token.position();
    Rc::new(Object::Error {
        message: message.to_string(),
        line: position.0,
        column: position.1,
    })
}

pub fn type_of(object: &Object) -> String {
    match object {
        Object::Int(_) => "Int".to_string(),
        Object::String(_) => "String".to_string(),
        Object::Boolean(_) => "Boolean".to_string(),
        Object::ReturnValue(_) => "ReturnValue".to_string(),
        Object::Error { .. } => "Error".to_string(),
        Object::Null => "Null".to_string(),
        Object::Function {
            parameters,
            body,
            env,
        } => {
            let params = join_collection!(parameters, ", ");
            return format!("Function  {}", params);
        }
        Object::Builtin(built_in_function) => {
            "BuiltInFunction: ".to_string() + &built_in_function.to_string()
        }
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
            Object::Function { .. } => type_of(self),
            Object::Builtin(built_in_function) => built_in_function.to_string(),
        }
    }
}
