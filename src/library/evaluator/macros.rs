#[macro_export]
macro_rules! control_flow_dependent {
    ($value:ident, $no_control_flow:expr) => {
        if let crate::object::Object::ReturnValue(_) = $value {
            return $value;
        }
        if let crate::object::Object::Error { .. } = $value {
            return $value;
        }
        return $no_control_flow;
    };
}
#[macro_export]
macro_rules! end_flow {
    ($value:ident ) => {
        if let crate::object::Object::ReturnValue(_) = $value {
            return $value;
        }
        if let crate::object::Object::Error { .. } = $value {
            return $value;
        }
    };
}
