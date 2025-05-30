use super::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: std::collections::HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.variables.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}
