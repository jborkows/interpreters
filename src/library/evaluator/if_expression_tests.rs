use super::evaluator_tests::{should_be_integer_equal_to, should_be_null};

#[test]
fn test_if_expressions() {
    should_be_integer_equal_to("if (true) { 10 }", 10);
    should_be_null("if (false) { 10 }");
    should_be_integer_equal_to("if (1 < 2) { 10 }", 10);
    should_be_null("if (1 > 2) { 10 }");
    should_be_integer_equal_to("if (1 < 2) { 10 } else { 20 }", 10);
    should_be_integer_equal_to("if (1 > 2) { 10 } else { 20 }", 20);
}
