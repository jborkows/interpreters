use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::{test_be_integer, test_compilation};
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

addition_of_two_integer: (
        "1+2",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1),test_be_integer(2)]),
multiple_expressions: (
    "1;2",
    vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Pop.into(), &[]),
    ],
        vec![test_be_integer(1),test_be_integer(2)]),
}
