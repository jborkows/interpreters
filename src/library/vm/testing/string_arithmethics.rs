use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_string},
};

generate_vm_tests! {
    one: (r#" "A"  "#, should_be_string("A")),
    adding_two: (r#" "A" + "B"  "#, should_be_string("AB")),
    adding_three: (r#" "A" + "B" + "C"  "#, should_be_string("ABC")),
    repeat_string: (r#" "A" * 3 "#, should_be_string("AAA")),
    integer_and_string: (r#" 3 + "A" * 2 "#, should_be_string("3AA")),
    string_and_integer: (r#"  "A" + 2 "#, should_be_string("A2")),

}
