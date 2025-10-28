use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    one: ("1", should_be_integer(1)),
    second: ("2", should_be_integer(2)),
    addition: ("1+2", should_be_integer(3)),
    substitution: ("1-2", should_be_integer(-1)),
    multiplication: ("2*3", should_be_integer(6)),
    division: ("12/3", should_be_integer(4)),
    complex: ("4+12/3-2*2+(1+2)*2", should_be_integer(10)),
}
