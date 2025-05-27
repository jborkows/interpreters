use std::any::Any;

pub(crate) trait Node: ToString + Any {
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! join_collection {
    ($expr:expr, $joiner:expr) => {
        $expr
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join($joiner)
    };
}

#[macro_export]
macro_rules! join_rc_collection {
    ($expr:expr, $joiner:expr) => {
        $expr
            .as_ref()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join($joiner)
    };
}

/**
* Don't know if use macro or function
*/
#[allow(dead_code)]
fn join_collection<T: ToString>(expr: &Vec<T>, joiner: &str) -> String {
    expr.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(joiner)
}
