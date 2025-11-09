use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::test_be_integer;
use crate::code::testing::test_compiler::test_compilation;
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

empty_array: (
        "[];1;",
        vec![
            make(OpCodes::Array.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1)]
    ),

an_array: (
    "[1,2,3]",
    vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Array.into(), &[3]),
            make(OpCodes::Pop.into(), &[]),
    ],
    vec![test_be_integer(1),test_be_integer(2), test_be_integer(3)]
    ),

array_of_elements: (
    "[1+1,2-1,3*4]",
    vec![
    make(OpCodes::Constant.into(), &[0]),
    make(OpCodes::Constant.into(), &[1]),
    make(OpCodes::Add.into(), &[]),
    make(OpCodes::Constant.into(), &[2]),
    make(OpCodes::Constant.into(), &[3]),
    make(OpCodes::Subtitute.into(), &[]),
    make(OpCodes::Constant.into(), &[4]),
    make(OpCodes::Constant.into(), &[5]),
    make(OpCodes::Multiply.into(), &[]),
    make(OpCodes::Array.into(), &[3]),
    make(OpCodes::Pop.into(), &[])
    ],
    vec![
        test_be_integer(1),
        test_be_integer(1),
        test_be_integer(2),
        test_be_integer(1),
        test_be_integer(3),
        test_be_integer(4)
    ]
),

}
