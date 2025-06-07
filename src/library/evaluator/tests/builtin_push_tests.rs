use crate::{expected_error_with_text, expected_integer_as_result_tests};

expected_integer_as_result_tests! {
    push_to_empty: ("push([],6)[1]", 6),
    push_to_single_keeps_original: ("push([0],5)[1]", 0),
    push_to_single_ads_new: ("push([0],3)[2]", 3),
}

expected_error_with_text! { "Invalid argument",
    {
        cannot_integer: "push(1,1)",
        cannot_boolean: "push(true, true)",
        cannot_string: "push(\"x\", \"y\")",
    }
}
expected_error_with_text! { "Function push expected 2 argument",
    {
        cannot_no_arguments: "push()",
        cannot_more_then_two: "push(\"x\", \"y\", \"z\")",
        cannot_less_then_two: "push([])",
    }
}
