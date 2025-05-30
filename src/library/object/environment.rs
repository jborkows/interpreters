use std::rc::Rc;

use super::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: std::collections::HashMap<String, Object>,
    outer: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            outer: None,
        }
    }
    pub fn enclosed(outer: Rc<Environment>) -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            outer: Some(outer.clone()),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.variables.get(name).or_else(|| {
            self.outer
                .as_ref()
                .and_then(|outer_env| outer_env.get(name))
        })
    }
}
