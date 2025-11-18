use std::hash::{Hash, Hasher};
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::ast::expression::Expression;
use crate::code::Instructions;
use crate::{ast::statements::Statement, join_collection, tokens::Token};
mod builtins;
mod environment;
mod helpers;
mod object_pool;
pub use builtins::BuiltInFunction;
pub use builtins::BuiltInResult;
pub use builtins::parse_built_in_function;
pub use environment::{Environment, new_environment};
pub use helpers::*;
#[cfg(test)]
mod testing;

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
    Macro {
        parameters: Vec<Identifier>,
        body: Rc<Statement>,
        env: Rc<RefCell<Environment>>,
    },
    Builtin(BuiltInFunction),
    Array {
        elements: Vec<Rc<Object>>,
    },
    //TODO: Implement collision mechanics, probably using a linked list
    HashMap(std::collections::HashMap<HashValue, Rc<HashEntry>>),
    Quote(Rc<Expression>),
    CompiledFunction {
        instructions: Instructions,
        number_of_locals: usize,
        number_of_parameters: usize,
    },
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
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

//TODO: replace with accepting position
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
            body: _,
            env: _,
        } => join_collection!(parameters, ", "),
        Object::Macro {
            parameters,
            body: _,
            env: _,
        } => "macro: ".to_string() + &(join_collection!(parameters, ", ")).to_string(),
        Object::Builtin(built_in_function) => {
            "BuiltInFunction: ".to_string() + &built_in_function.to_string()
        }
        Object::Array { .. } => "Array".to_string(),
        Object::HashMap(_) => "HashMap".to_string(),
        Object::Quote(_) => "Quote: ".to_string(),
        Object::CompiledFunction {
            instructions: _,
            number_of_locals: _,
            number_of_parameters: number_of_arguments,
        } => format!("CompiledFunction({number_of_arguments})"),
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(i) => write!(f, "{}", i),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Null => write!(f, "NULL"),
            Object::ReturnValue(object) => write!(f, "{}", object),
            Object::Error {
                message,
                line,
                column,
            } => write!(f, "Error at {}:{} -> {}", line, column, message),
            Object::Function { .. } => write!(f, "{}", type_of(self)),
            Object::Macro { .. } => write!(f, "{}", type_of(self)),
            Object::Builtin(built_in_function) => write!(f, "{}", built_in_function),
            Object::Array { elements } => {
                let elements_str: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", join_collection!(elements_str, ", "))
            }
            Object::HashMap(map) => {
                let entries: Vec<String> = map
                    .iter()
                    .map(|(_, entry)| format!("{}: {}", entry.key, entry.value))
                    .collect();
                write!(f, "{{{}}}", join_collection!(entries, ", "))
            }
            Object::Quote(statement) => write!(f, "Quote: {}", statement),
            Object::CompiledFunction {
                instructions,
                number_of_locals,
                number_of_parameters: number_of_arguments,
            } => write!(
                f,
                "CompiledFunction({number_of_arguments}, instructions:{instructions}, locals: {number_of_locals})"
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashValue(i64);
impl Display for HashValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash({})", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct HashEntry {
    pub key: Rc<Object>,
    pub value: Rc<Object>,
}

pub fn hash(object: &Object) -> HashValue {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    match object {
        Object::Int(i) => i.hash(&mut hasher),
        Object::String(s) => s.hash(&mut hasher),
        Object::Boolean(b) => b.hash(&mut hasher),
        Object::ReturnValue(rv) => panic!("Cannot, hash ReturnValue directly: {}", rv),
        Object::Error {
            message,
            line,
            column,
        } => {
            message.hash(&mut hasher);
            line.hash(&mut hasher);
            column.hash(&mut hasher);
        }
        Object::Null => 0.hash(&mut hasher),
        Object::Function {
            parameters,
            body: _,
            env: _,
        } => {
            parameters.iter().for_each(|p| p.name.hash(&mut hasher));
        }

        Object::Macro {
            parameters,
            body: _,
            env: _,
        } => {
            parameters.iter().for_each(|p| p.name.hash(&mut hasher));
            "macro".hash(&mut hasher);
        }
        Object::Builtin(built_in_function) => built_in_function.to_string().hash(&mut hasher),
        Object::Array { elements } => {
            elements.iter().for_each(|e| hash(e).0.hash(&mut hasher));
        }
        Object::HashMap(map) => panic!("Cannot hash HashMap directly: {}", map.len()),
        Object::Quote(statement) => panic!("Cannot hash Quote directly: {}", statement),
        Object::CompiledFunction {
            instructions,
            number_of_locals: _,
            number_of_parameters: number_of_arguments,
        } => panic!("Cannot hash CompiledFunction directly: {instructions} {number_of_arguments}"),
    }
    HashValue(hasher.finish() as i64)
}
