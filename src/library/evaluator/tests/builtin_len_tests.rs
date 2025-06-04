use crate::{expected_error_with_text, expected_integer_as_result_tests};

expected_integer_as_result_tests! {
    len: ("len(\"xx\")", 2),
    len_with_spaces: ("len(\"xx  \")", 4),
    len_expression: ("len(\"x\"*3)", 3),
    len_for_array: ("len([1, 2, 3])", 3),
}

expected_error_with_text! { "Invalid argument",
    {
        cannot_len_integer: "len(1)",
        cannot_len_boolean: "len(true)",
    }
}
expected_error_with_text! { "Function len expected 1 argument",
    {
        cannot_len_no_arguments: "len()",
        cannot_len_for_more_than_one: "len(\"x\", \"y\")",
        cannot_len_for_more_than_two: "len(\"x\", \"y\", \"z\")",
    }
}
