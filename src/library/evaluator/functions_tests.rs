use crate::expected_integer_as_result_tests;

use super::evaluator_tests::eval_input;

#[test]
fn can_construct_function() {
    let input = r#"
        fn(a, b) {
            a + b
        };
    "#;
    let object = eval_input(input);
    match object.as_ref() {
        crate::object::Object::Function {
            parameters,
            body,
            env: _,
        } => {
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0].to_string(), "a");
            assert_eq!(parameters[1].to_string(), "b");
            assert_eq!(body.to_string(), "(a + b)");
        }
        _ => panic!(
            "Expected a Function object, but got: {}",
            object.to_string()
        ),
    }
}

expected_integer_as_result_tests! {
    simple: ("let identity = fn(x){x;}; identity(5);", 5),
    simple_return: ("let identity = fn(x){return x;}; identity(5);", 5),
    double: ("let double = fn(x){return x*2;}; identity(5);", 10),
    add: ("let add = fn(x, y){return x + y;}; add(5, 10);", 15),
    add_fun: ("let add = fn(x, y){return x + y;}; add(5+5, add(5,5));", 20),
    anonymous: ("fn(x){x;}(5);", 5),
}
