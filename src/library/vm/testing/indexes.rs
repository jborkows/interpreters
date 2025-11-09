use core::panic;

use crate::{
    generate_vm_tests, vm::testing::setups::run_vm_test, vm::testing::setups::should_be_integer,
    vm::testing::setups::should_be_null,
};

generate_vm_tests! {
    empty_map: ("{}[1]", should_be_null()),
    empty_array: ("[][1]", should_be_null()),
    one_based_index: ("[4,2,3][1]", should_be_integer(4)),
    one_based_reversed_index: ("[4,2,3][-1]", should_be_integer(3)),
    outside_index: ("[4,2,3][4]", should_be_null()),
    deep_index: ("[[4,2,3]][1][3]", should_be_integer(3)),
    not_exisitng_key: ("{4:2,2:3,3:1}[0]", should_be_null()),
    existing_key: ("{4:2,2:3,3:1}[2]", should_be_integer(3)),
}
