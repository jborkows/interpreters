use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    single: ("let x = 1;x", should_be_integer(1)),
    two: ("let x = 1;let y = 2;x+y", should_be_integer(3)),
    reusing: ("let x = 1;let y = x+x;x+y", should_be_integer(3)),
    reusing_with_abondoning: ("let x = 1;let y = x+x;x;x+y", should_be_integer(3)),
}
