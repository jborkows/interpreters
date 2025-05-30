use crate::expected_integer_as_result_tests;

use super::evaluator_tests::should_be_error_with_text;

expected_integer_as_result_tests! {
    let_simple: ("let x = 10; x;", 10),
    let_expression: ("let x = 10*10; x;", 100),
    let_combination: ("let a=5;let b=a;b", 5),
    let_complex: ("let a=5;let b=10;let c=a+b;c", 15),
}

#[test]
fn identifier_not_found() {
    let error_text = "Identifier 'x' not found";
    should_be_error_with_text("x;", error_text);
    should_be_error_with_text("x; 1;", error_text);
}
