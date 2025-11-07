use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer, should_be_null},
};

generate_vm_tests! {
    single: ("if(true){10}", should_be_integer(10)),
    if_else_always_true: ("if(true){10} else {20}", should_be_integer(10)),
    if_else_always_false: ("if(false){10} else {20}", should_be_integer(20)),
    if_else_always_condition_true: ("if(1 < 2){10} else {20}", should_be_integer(10)),
    if_else_always_condition_false: ("if(1 > 2){10} else {20}", should_be_integer(20)),
    if_false_else_not_defined: ("if(1 > 2){10}", should_be_null()),

}
