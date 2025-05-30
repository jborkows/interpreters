use super::evaluator_tests::{should_be_boolean_equal_to, should_be_integer_equal_to};

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
