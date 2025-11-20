use std::{fmt::Display, rc::Rc};

use crate::object::{Object, int_value};

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
macro_rules! expecting_array {
    ($left:ident,  $function_name:expr, $argument_no:expr ) => {
        (match $left.as_ref() {
            super::Object::Array { elements } => Ok(elements),
            _ => {
                let error_message = format!(
                    "Invalid argument {} for {}: {}({}) expected Array",
                    $argument_no,
                    $function_name,
                    super::type_of($left),
                    $left.to_string(),
                );
                Err(BuiltInResult::Failure(error_message))
            }
        })
    };
}

macro_rules! value {
    ($value:expr  ) => {
        BuiltInResult::Value(Rc::new($value))
    };
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BuiltInFunction {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
    Quote,
}

pub enum BuiltInResult {
    Unit,
    Value(Rc<Object>),
    Failure(String),
}

const BUILTINS_DATA: [BuiltInFunction; 6] = [
    BuiltInFunction::Len,
    BuiltInFunction::First,
    BuiltInFunction::Last,
    BuiltInFunction::Rest,
    BuiltInFunction::Push,
    BuiltInFunction::Puts,
];
impl BuiltInFunction {
    //TODO: replace to return Result either object or error ready structure -> function accepting
    //position
    pub(crate) fn apply(&self, arguments: &[Rc<super::Object>]) -> BuiltInResult {
        match self {
            BuiltInFunction::Len => apply_len(arguments),
            BuiltInFunction::First => apply_first(arguments),
            BuiltInFunction::Last => apply_last(arguments),
            BuiltInFunction::Rest => apply_rest(arguments),
            BuiltInFunction::Push => apply_push(arguments),
            BuiltInFunction::Puts => apply_puts(arguments),
            BuiltInFunction::Quote => apply_quote(arguments),
        }
    }
    pub fn index(&self) -> u8 {
        return BUILTINS_DATA
            .iter()
            .position(|x| x == self)
            .expect(&format!(
                "It seams {} was not fully defined, lucking index",
                &self
            )) as u8;
    }

    pub fn decode(index: usize) -> BuiltInFunction {
        return BUILTINS_DATA
            .get(index)
            .expect(&format!("Cannot decode {index}"))
            .clone();
    }

    pub fn all<'a>() -> &'a [BuiltInFunction] {
        &[
            BuiltInFunction::Len,
            BuiltInFunction::First,
            BuiltInFunction::Last,
            BuiltInFunction::Rest,
            BuiltInFunction::Push,
            BuiltInFunction::Puts,
        ]
    }
}

fn apply_quote(arguments: &[std::rc::Rc<super::Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("quote", 1, arguments));
    todo!()
}

fn apply_puts(arguments: &[std::rc::Rc<super::Object>]) -> BuiltInResult {
    arguments.into_iter().for_each(|arg| {
        println!(">> {}", arg.to_string());
    });
    BuiltInResult::Unit
}

fn apply_push(arguments: &[std::rc::Rc<super::Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("push", 2, arguments));
    let an_array = &arguments[0];
    let value = end_flow!(expecting_array!(an_array, "push", 1));
    let new_value = &arguments[1];
    let mut copied = value.clone();
    copied.push(new_value.clone());
    value!(Object::Array { elements: copied })
}

fn apply_rest(arguments: &[std::rc::Rc<super::Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("rest", 1, arguments));
    let argument = &arguments[0];
    let value = end_flow!(expecting_array!(argument, "rest", 1));
    if value.is_empty() {
        return value!(Object::Array { elements: vec![] });
    }
    value
        .get(1..)
        .map(|slice| {
            value!(Object::Array {
                elements: slice.to_vec()
            })
        })
        .unwrap_or_else(|| {
            BuiltInResult::Failure("Cannot get rest of array for empty array".to_string())
        })
}

fn apply_last(arguments: &[Rc<Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("last", 1, arguments));
    let argument = &arguments[0];
    let value = end_flow!(expecting_array!(argument, "last", 1));
    value
        .into_iter()
        .last()
        .cloned()
        .map(|x| BuiltInResult::Value(x))
        .unwrap_or_else(|| {
            BuiltInResult::Failure("Cannot get last element for empty array".to_string())
        })
}

fn apply_first(arguments: &[Rc<Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("first", 1, arguments));
    let argument = &arguments[0];
    let value = end_flow!(expecting_array!(argument, "first", 1));
    value
        .get(0)
        .cloned()
        .map(|x| BuiltInResult::Value(x))
        .unwrap_or_else(|| {
            BuiltInResult::Failure("Cannot get first element for empty array".to_string())
        })
}

fn apply_len(arguments: &[Rc<Object>]) -> BuiltInResult {
    end_flow!(accept_n_arguments("len", 1, arguments));
    let argument = &arguments[0];
    match argument.as_ref() {
        super::Object::Array { elements } => BuiltInResult::Value(int_value(elements.len() as i64)),
        super::Object::String(s) => BuiltInResult::Value(int_value(s.len() as i64)),
        super::Object::Int(_) | super::Object::Boolean(_) | super::Object::Null => {
            return BuiltInResult::Failure(format!(
                "Invalid argument for len: {}({}) expected Array or String",
                super::type_of(argument),
                argument.to_string()
            ));
        }
        _ => {
            return BuiltInResult::Failure(format!(
                "Invalid argument for len: {}({}) expected Array or String",
                super::type_of(argument),
                argument.to_string()
            ));
        }
    }
}

fn accept_n_arguments(
    name: &str,
    expected: usize,
    arguments: &[Rc<Object>],
) -> Result<(), BuiltInResult> {
    if arguments.len() != expected {
        let message = format!(
            "Function {} expected {} arguments, got {}",
            name,
            expected,
            arguments.len()
        );
        return Err(BuiltInResult::Failure(message.clone()));
    }
    Ok(())
}

pub fn parse_built_in_function(function_name: &str) -> Option<BuiltInFunction> {
    match function_name {
        "len" => Some(BuiltInFunction::Len),
        "last" => Some(BuiltInFunction::Last),
        "first" => Some(BuiltInFunction::First),
        "rest" => Some(BuiltInFunction::Rest),
        "push" => Some(BuiltInFunction::Push),
        "puts" => Some(BuiltInFunction::Puts),
        "quote" => Some(BuiltInFunction::Quote),
        _ => None,
    }
}

impl Display for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltInFunction::Len => write!(f, "len"),
            BuiltInFunction::First => write!(f, "first"),
            BuiltInFunction::Last => write!(f, "last"),
            BuiltInFunction::Rest => write!(f, "rest"),
            BuiltInFunction::Push => write!(f, "push"),
            BuiltInFunction::Puts => write!(f, "puts"),
            BuiltInFunction::Quote => write!(f, "quote"),
        }
    }
}
