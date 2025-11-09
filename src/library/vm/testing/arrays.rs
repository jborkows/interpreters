use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer_array},
};

generate_vm_tests! {
    empty: ("[]", should_be_integer_array(&[])),
    numbers: ("[1,2,3]", should_be_integer_array(&[1,2,3])),
    calculated: ("[1 + 2, 3 * 4, 5 + 6]", should_be_integer_array(&[3,12,11])),

}
