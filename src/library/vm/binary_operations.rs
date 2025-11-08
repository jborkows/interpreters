use crate::vm::wrap_boolean;
use crate::{ast::expression::InfixOperatorType, object::Object};

pub(crate) fn binary(left: Object, right: Object, operator: InfixOperatorType) -> Object {
    match right {
        Object::Int(r) => match left {
            Object::Int(l) => {
                let operator = operator;
                let left = l;
                let right = r;
                match operator {
                    InfixOperatorType::Plus => Object::Int(left + right),
                    InfixOperatorType::Minus => Object::Int(left - right),
                    InfixOperatorType::Multiply => Object::Int(left * right),
                    InfixOperatorType::Divide => Object::Int(left / right),
                    InfixOperatorType::NotEqual => wrap_boolean(left != right),
                    InfixOperatorType::GreaterThan => wrap_boolean(left > right),
                    InfixOperatorType::Equal => wrap_boolean(left == right),
                    _ => panic!(
                        "Don't know how to deal with {right:?} and {left:?} for {operator:?}"
                    ),
                }
            }
            Object::String(l) => match operator {
                InfixOperatorType::Plus => Object::String(l + &r.to_string()),
                InfixOperatorType::Multiply => Object::String(l.repeat(r as usize)),
                _ => panic!("Don't know how to deal with {right:?} and {l:?} for {operator:?}"),
            },

            _ => panic!("Don't know how to deal with {right:?} and {left:?} for {operator:?}"),
        },
        Object::Boolean(r) => match left {
            Object::Boolean(l) => match operator {
                InfixOperatorType::NotEqual => wrap_boolean(l != r),
                InfixOperatorType::Equal => wrap_boolean(l == r),
                _ => panic!("Don't know how to deal with {right:?} and {left:?} for {operator:?}"),
            },
            _ => panic!("Don't know how to deal with {right:?} and {left:?} for {operator:?}"),
        },
        Object::String(r) => match left {
            Object::String(l) => match operator {
                InfixOperatorType::Plus => Object::String(l + &r),
                _ => panic!("Don't know how to deal with {l:?} and {r:?} for {operator:?}"),
            },
            Object::Int(l) => match operator {
                InfixOperatorType::Plus => Object::String(l.to_string() + &r),
                _ => panic!("Don't know how to deal with {l:?} and {r:?} for {operator:?}"),
            },

            _ => panic!(
                "Don't know how to deal with {left:?} and {:?} for {operator:?}",
                r.clone(),
            ),
        },

        _ => panic!("Don't know how to deal with {right:?} for {operator:?}"),
    }
}
