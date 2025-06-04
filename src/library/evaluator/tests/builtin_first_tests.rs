use crate::{expected_error_with_text, expected_integer_as_result_tests};

expected_integer_as_result_tests! {
    first: ("first([5, 2, 3])", 5),
}

expected_error_with_text! { "Invalid argument",
    {
        cannot_first_integer: "first(1)",
        cannot_first_boolean: "first(true)",
        cannot_first_string: "first(\"x\")",
    }
}
expected_error_with_text! { "Function first expected 1 argument",
    {
        cannot_len_no_arguments: "first()",
        cannot_len_for_more_than_one: "first(\"x\", \"y\")",
        cannot_len_for_more_than_two: "first(\"x\", \"y\", \"z\")",
    }
}
expected_error_with_text! { "Cannot get first element for empty array",
    {
        cannot_first_empty_array: "first([])",
    }
}
