use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_error, should_be_integer, should_be_null},
};

generate_vm_tests! {
    len_empty: (r#"len("")"#, should_be_integer(0)),
    len_short_string: (r#"len("four")"#, should_be_integer(4)),
    len_string: (r#"len("hello world")"#, should_be_integer(11)),
    len_boolean: (r#"len(false)"#, should_be_error(|_x|Result::Ok(()))),
    first_of_empty: (r#"first([])"#, should_be_error(|_x|Result::Ok(()))),
    first_of_something: (r#"first([1,2])"#, should_be_integer(1)),
}
