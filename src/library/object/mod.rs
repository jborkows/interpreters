#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    String(String),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
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
        }
    }
}
