use crate::{expected_error_with_text, expected_integer_as_result_tests};

expected_integer_as_result_tests! {
    last: ("last([1, 2, 6])", 6),
}

expected_error_with_text! { "Invalid argument",
    {
        cannot_last_integer: "last(1)",
        cannot_last_boolean: "last(true)",
        cannot_last_string: "last(\"x\")",
    }
}
expected_error_with_text! { "Function last expected 1 argument",
    {
        cannot_len_no_arguments: "last()",
        cannot_len_for_more_than_one: "last(\"x\", \"y\")",
        cannot_len_for_more_than_two: "last(\"x\", \"y\", \"z\")",
    }
}
expected_error_with_text! { "Cannot get last element for empty array",
    {
        cannot_last_empty_array: "last([])",
    }
}
