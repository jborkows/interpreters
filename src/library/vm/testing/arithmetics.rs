use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    one: ("1", should_be_integer(1)),
    second: ("2", should_be_integer(2)),
    addition: ("1+2", should_be_integer(3)),
}
