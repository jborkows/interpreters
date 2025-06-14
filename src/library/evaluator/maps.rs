use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::Expression,
    end_flow,
    object::{Environment, HashEntry, Object, hash},
};

use super::evaluate_expression;

pub(crate) fn parse_map_literal(
    elements: &[(Expression, Expression)],
    environment: Rc<RefCell<Environment>>,
) -> Rc<Object> {
    let mut map = std::collections::HashMap::new();
    for (key, value) in elements {
        let key_value = evaluate_expression(key, environment.clone());
        end_flow!(key_value);
        let value_value = evaluate_expression(value, environment.clone());
        end_flow!(value_value);
        let hash_key_value = hash(&key_value);
        let hash_entry = HashEntry {
            key: key_value.clone(),
            value: value_value.clone(),
        };
        map.insert(hash_key_value, Rc::new(hash_entry));
    }
    Rc::new(Object::HashMap(map))
}
