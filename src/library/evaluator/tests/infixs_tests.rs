use super::evaluator_tests::{
    should_be_boolean_equal_to, should_be_error, should_be_integer_equal_to,
    should_be_string_equal_to,
};

#[test]
fn test_integer_calculations() {
    should_be_integer_equal_to("5 + 5", 10);
    should_be_integer_equal_to("10 - 5", 5);
    should_be_integer_equal_to("2 * 3", 6);
    should_be_integer_equal_to("8 / 2", 4);
    should_be_integer_equal_to("2 + 3 * 4", 14);
    should_be_integer_equal_to("(2 + 3) * 4", 20);
    should_be_integer_equal_to("-5 + 10 -5", 0);
    should_be_error("5 + true;2");
}

#[test]
fn test_logical_integer_expressions() {
    should_be_boolean_equal_to("1 < 2", true);
    should_be_boolean_equal_to("3 < 2", false);
    should_be_boolean_equal_to("1 > 2", false);
    should_be_boolean_equal_to("3 > 2", true);
    should_be_boolean_equal_to("(1+1) == 2", true);
    should_be_boolean_equal_to("(1+1) != 2", false);
    should_be_boolean_equal_to("1 != 2", true);
    should_be_boolean_equal_to("2 != 2", false);
}

#[test]
fn test_logical_boolean_expressions() {
    should_be_boolean_equal_to("true == true", true);
    should_be_boolean_equal_to("true != true", false);
    should_be_boolean_equal_to("true == false", false);
    should_be_boolean_equal_to("false == false", true);
    should_be_error("false > true;1");
    should_be_error("false < true;1");
}

#[test]
fn test_string_calculations() {
    should_be_string_equal_to("\"x\"+\"y\"", "xy".to_string());
    should_be_string_equal_to("\"x\"*3", "xxx".to_string());
    should_be_string_equal_to("\"x\"+3", "x3".to_string());
    should_be_string_equal_to("3+\"x\"", "3x".to_string());
    should_be_string_equal_to("true+\"x\"", "truex".to_string());
    should_be_string_equal_to("\"x\"+true", "xtrue".to_string());
}
#[test]
fn test_failed_string_calculations() {
    should_be_error("\"x\"-\"y\";1");
    should_be_error("\"x\"*\"y\";");
    should_be_error("4*\"y\";");
    should_be_error("\"x\"/\"y\";");
}

#[test]
fn test_string_logical_calculations() {
    should_be_boolean_equal_to("\"x\" == \"y\"", false);
    should_be_boolean_equal_to("\"x\" == \"x\"", true);
}
