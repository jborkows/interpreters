use crate::{
    current_allocation_counting, evaluator::tests::evaluator_tests::should_be_integer_equal_to,
};

use super::evaluator_tests::{should_be_boolean_equal_to, should_be_string_equal_to};

#[test]
fn text_evalaution_of_integers() {
    current_allocation_counting!();
    should_be_integer_equal_to("0", 0);
    should_be_integer_equal_to("1", 1);
    should_be_integer_equal_to("3", 3);
}

#[test]
fn text_evaluation_of_booleans() {
    should_be_boolean_equal_to("true", true);
    should_be_boolean_equal_to("false", false);
}

#[test]
fn text_evaluation_of_strings() {
    should_be_string_equal_to("\"Hello, World!\"", "Hello, World!".to_string());
    should_be_string_equal_to("\"Test String\"", "Test String".to_string());
}
