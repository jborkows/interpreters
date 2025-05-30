use super::evaluator_tests::should_be_integer_equal_to;

#[test]
fn test_return() {
    should_be_integer_equal_to("return 10;", 10);
    should_be_integer_equal_to("1;return 2;3", 2);
}
