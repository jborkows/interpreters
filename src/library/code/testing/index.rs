use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::test_be_integer;
use crate::code::testing::test_compiler::test_compilation;
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

index: (
        "[1,4,9][2]",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Array.into(), &[3]),
            make(OpCodes::Constant.into(), &[3]),
            make(OpCodes::Index.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1),test_be_integer(4), test_be_integer(9),test_be_integer(2)]
    ),

index_calculated: (
        "[1,4,9][1+2]",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Array.into(), &[3]),
            make(OpCodes::Constant.into(), &[3]),
            make(OpCodes::Constant.into(), &[4]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::Index.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1), test_be_integer(4),test_be_integer(9),test_be_integer(1), test_be_integer(2)]
    ),



}
