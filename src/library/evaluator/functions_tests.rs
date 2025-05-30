use super::evaluator_tests::eval_input;

#[test]
fn can_construct_function() {
    let input = r#"
        fn(a, b) {
            a + b
        };
    "#;
    let object = eval_input(input);
    match object {
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
