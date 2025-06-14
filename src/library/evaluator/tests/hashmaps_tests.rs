use crate::{
    evaluator::tests::evaluator_tests::eval_input, expected_boolean_object_to_be_equal,
    expected_integer_as_result_tests, expected_integer_value_to_be, expected_string_to_be_equal,
};

macro_rules! expected_hash_map {
    ($input:expr) => {{
        let object = eval_input($input);
        match object.as_ref() {
            crate::object::Object::HashMap(backing) => backing
                .iter()
                .next()
                .map(|(_key, value)| value.clone())
                .map(|hash_entry| (hash_entry.clone()))
                .map(|entry| (entry.key.clone(), entry.value.clone()))
                .unwrap(),
            _ => panic!("Expected a Hash object, but got: {}", object.to_string()),
        }
    }};
}

#[test]
fn string_can_be_a_key() {
    let (key, value) = expected_hash_map!(r#"{"one": 12-3}"#);
    expected_string_to_be_equal!(key, "one");
    expected_integer_value_to_be!(*value, 9);
}

#[test]
fn key_comes_from_identifier() {
    let (key, value) = expected_hash_map!(r#"let key="AKey";{key: 1+1}"#);
    expected_string_to_be_equal!(key, "AKey");
    expected_integer_value_to_be!(*value, 2);
}

#[test]
fn key_could_be_false_boolean() {
    let (key, value) = expected_hash_map!(r#" {false: 1+2}"#);
    expected_boolean_object_to_be_equal!(key, false);
    expected_integer_value_to_be!(*value, 3);
}

#[test]
fn key_could_be_true_boolean() {
    let (key, value) = expected_hash_map!(r#" {true: 2+2}"#);
    expected_boolean_object_to_be_equal!(key, true);
    expected_integer_value_to_be!(*value, 4);
}

#[test]
fn integer_can_be_a_key() {
    let (key, value) = expected_hash_map!(r#" {1: 2+2}"#);
    expected_integer_value_to_be!(*key, 1);
    expected_integer_value_to_be!(*value, 4);
}

expected_integer_as_result_tests! {
    getting_by_string_key: (r#"let key="foo";{"foo": 2+2}[key]"#, 4),
    getting_by_number_key: (r#"{2: 6}[2]"#, 6),
    getting_by_boolean_key: (r#"{true: 6, false:4}[false]"#, 4),
}

#[test]
fn expecting_null_when_key_not_found() {
    let object = eval_input(r#"{}["bar"]"#);
    match object.as_ref() {
        crate::object::Object::Null => {}
        _ => panic!("Expected a Null object, but got: {}", object.to_string()),
    }
}
