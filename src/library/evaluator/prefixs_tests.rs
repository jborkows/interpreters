use super::evaluator_tests::{
    should_be_boolean_equal_to, should_be_error_with_text, should_be_integer_equal_to,
};

#[test]
fn test_bang_operator() {
    should_be_boolean_equal_to("!\"\"", true);
    should_be_boolean_equal_to("!\"  \"", true);
    should_be_boolean_equal_to("!\"xx\"", false);
    should_be_boolean_equal_to("!!\"xx\"", true);
    should_be_boolean_equal_to("!5", false);
    should_be_boolean_equal_to("!0", false);
    should_be_boolean_equal_to("!!0", true);
    should_be_boolean_equal_to("!!5", true);
    should_be_boolean_equal_to("!!true", true);
    should_be_boolean_equal_to("!!false", false);
}

#[test]
fn test_minus_operator() {
    should_be_integer_equal_to("-5", -5);
    should_be_integer_equal_to("-10", -10);
}

#[test]
fn test_wrong_prefix_boolean_operation() {
    let error_text = "Minus (-) cannot be applied to Boolean";
    should_be_error_with_text("-false", error_text);
    should_be_error_with_text("-false;true", error_text);
}

#[test]
fn test_wrong_prefix_string_operation() {
    let minus = "Minus (-) cannot be applied to String";

    should_be_error_with_text("-\"aaa\";", minus);
    should_be_error_with_text("-\"aaa\";2", minus);
}
