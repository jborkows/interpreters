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
    ($left:ident, $token:expr, $argument_no:expr, $variant:ident) => {
        (match $left.as_ref() {
            super::Object::$variant(s) => Ok(s),
            _ => Err(error_at(
                format!(
                    "Invalid argument {}: {}({}) expected {}",
                    $argument_no,
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

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Len,
}
impl BuiltInFunction {
    pub(crate) fn apply(
        &self,
        token: &Token,
        arguments: &[std::rc::Rc<super::Object>],
    ) -> std::rc::Rc<super::Object> {
        match self {
            BuiltInFunction::Len => apply_len(token, arguments),
        }
    }
}

fn apply_len(
    token: &Token,
    arguments: &[std::rc::Rc<super::Object>],
) -> std::rc::Rc<super::Object> {
    end_flow!(accept_n_arguments("len", 1, token, arguments));
    let argument = &arguments[0];
    let value = end_flow!(argument_should_be!(argument, token, 1, String));
    int_value(value.len() as i64)
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
        _ => None,
    }
}

impl ToString for BuiltInFunction {
    fn to_string(&self) -> String {
        match self {
            BuiltInFunction::Len => "len".to_string(),
        }
    }
}
