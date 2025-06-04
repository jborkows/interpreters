use std::fmt::Display;

use crate::{object::int_value, tokens::Token};

use super::error_at;

macro_rules! end_flow {
    ($value:expr ) => {
        match $value {
            Err(e) => {
                return e;
            }
            Ok(v) => v,
        }
    };
}
macro_rules! argument_should_be {
    ($left:ident, $token:expr, $function_name:expr, $argument_no:expr, $variant:ident) => {
        (match $left.as_ref() {
            super::Object::$variant(s) => Ok(s),
            _ => Err(error_at(
                format!(
                    "Invalid argument {} for {}: {}({}) expected {}",
                    $argument_no,
                    $function_name,
                    super::type_of($left),
                    $left.to_string(),
                    stringify!($variant)
                )
                .as_str(),
                $token,
            )),
        })
    };
}
macro_rules! expecting_array {
    ($left:ident, $token:expr, $function_name:expr, $argument_no:expr ) => {
        (match $left.as_ref() {
            super::Object::Array { elements } => Ok(elements),
            _ => Err(error_at(
                format!(
                    "Invalid argument {} for {}: {}({}) expected Array",
                    $argument_no,
                    $function_name,
                    super::type_of($left),
                    $left.to_string(),
                )
                .as_str(),
                $token,
            )),
        })
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Len,
    First,
    Last,
}
impl BuiltInFunction {
    pub(crate) fn apply(
        &self,
        token: &Token,
        arguments: &[std::rc::Rc<super::Object>],
    ) -> std::rc::Rc<super::Object> {
        match self {
            BuiltInFunction::Len => apply_len(token, arguments),
            BuiltInFunction::First => apply_first(token, arguments),
            BuiltInFunction::Last => apply_last(token, arguments),
        }
    }
}

fn apply_last(
    token: &Token,
    arguments: &[std::rc::Rc<super::Object>],
) -> std::rc::Rc<super::Object> {
    end_flow!(accept_n_arguments("last", 1, token, arguments));
    let argument = &arguments[0];
    let value = end_flow!(expecting_array!(argument, token, "last", 1));
    value
        .into_iter()
        .last()
        .cloned()
        .unwrap_or_else(|| error_at("Cannot get last element for empty array", token))
}

fn apply_first(
    token: &Token,
    arguments: &[std::rc::Rc<super::Object>],
) -> std::rc::Rc<super::Object> {
    end_flow!(accept_n_arguments("first", 1, token, arguments));
    let argument = &arguments[0];
    let value = end_flow!(expecting_array!(argument, token, "first", 1));
    value
        .get(0)
        .cloned()
        .unwrap_or_else(|| error_at("Cannot get first element for empty array", token))
}

fn apply_len(
    token: &Token,
    arguments: &[std::rc::Rc<super::Object>],
) -> std::rc::Rc<super::Object> {
    end_flow!(accept_n_arguments("len", 1, token, arguments));
    let argument = &arguments[0];
    match argument.as_ref() {
        super::Object::Array { elements } => int_value(elements.len() as i64),
        super::Object::String(s) => int_value(s.len() as i64),
        super::Object::Int(_) | super::Object::Boolean(_) | super::Object::Null => {
            return error_at(
                format!(
                    "Invalid argument for len: {}({}) expected Array or String",
                    super::type_of(argument),
                    argument.to_string()
                )
                .as_str(),
                token,
            );
        }
        _ => {
            return error_at(
                format!(
                    "Invalid argument for len: {}({}) expected Array or String",
                    super::type_of(argument),
                    argument.to_string()
                )
                .as_str(),
                token,
            );
        }
    }
}

fn accept_n_arguments(
    name: &str,
    expected: usize,
    token: &Token,
    arguments: &[std::rc::Rc<super::Object>],
) -> Result<(), std::rc::Rc<super::Object>> {
    if arguments.len() != expected {
        return Err(error_at(
            format!(
                "Function {} expected {} arguments, got {}",
                name,
                expected,
                arguments.len()
            )
            .as_str(),
            token,
        ));
    }
    Ok(())
}

pub fn parse_built_in_function(function_name: &str) -> Option<BuiltInFunction> {
    match function_name {
        "len" => Some(BuiltInFunction::Len),
        "last" => Some(BuiltInFunction::Last),
        "first" => Some(BuiltInFunction::First),
        _ => None,
    }
}

impl Display for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltInFunction::Len => write!(f, "len"),
            BuiltInFunction::First => write!(f, "first"),
            BuiltInFunction::Last => write!(f, "last"),
        }
    }
}
