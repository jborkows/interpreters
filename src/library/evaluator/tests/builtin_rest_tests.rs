use crate::{expected_error_with_text, expected_integer_as_result_tests};

expected_integer_as_result_tests! {
    rest_contains_last: ("rest([1, 2, 6])[2]", 6),
    rest_contains_previous: ("rest([1, 2, 6])[1]", 2),
    rest_can_be_called_multiple_times: ("len(rest(rest(rest([1,2]))))", 0),
}

expected_error_with_text! { "Invalid argument",
    {
        cannot_integer: "rest(1)",
        cannot_boolean: "rest(true)",
        cannot_string: "rest(\"x\")",
    }
}
expected_error_with_text! { "Function rest expected 1 argument",
    {
        cannot_no_arguments: "rest()",
        cannot_for_more_than_one: "rest(\"x\", \"y\")",
        cannot_for_more_than_two: "rest(\"x\", \"y\", \"z\")",
    }
}
