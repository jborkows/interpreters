use std::{cell::RefCell, rc::Rc};

use super::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: std::collections::HashMap<String, Rc<Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            outer: None,
        }
    }
    pub fn enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            outer: Some(outer.clone()),
        }
    }

    pub fn set(&mut self, name: String, value: Rc<Object>) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        if let Some(value) = self.variables.get(name) {
            return Some(value.clone());
        }
        if let Some(outer_env) = &self.outer {
            let outer_env = outer_env.clone();
            return outer_env.borrow_mut().get(name);
        }
        None
    }
}

pub fn new_environment() -> Rc<RefCell<Environment>> {
    Rc::new(RefCell::new(crate::object::Environment::new()))
}
