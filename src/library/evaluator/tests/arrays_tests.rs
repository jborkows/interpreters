use crate::{expected_error_with_text, expected_integer_as_result_tests};

use super::evaluator_tests::eval_input;

#[test]
fn can_constuct_array() {
    let input = "[1, 2+3, 4*5];";
    let object = eval_input(input);
    match object.as_ref() {
        crate::object::Object::Array { elements } => {
            assert_eq!(elements.len(), 3);
            assert_eq!(*elements[0], crate::object::Object::Int(1));
            assert_eq!(*elements[1], crate::object::Object::Int(5));
            assert_eq!(*elements[2], crate::object::Object::Int(20));
        }
        _ => panic!("Expected an array object, but got: {}", object.to_string()),
    }
}

expected_integer_as_result_tests! {
    starting_from_1: ("[1,2+3,3+5][1]", 1),
    starting_second_is_second: ("[1,2+3,3+5][2]", 5),
    with_asssignment: ("let a = [1,2+3,3+5]; a[1]", 1),
    negative_will_go_from_last: ("[1,2+3,3+5][-1]", 8),
}

expected_error_with_text! { "Index out of bounds",{
        above_len_leads_to_error: "[1][2]",
        is_1_based: "[1][0]",
    }
}
