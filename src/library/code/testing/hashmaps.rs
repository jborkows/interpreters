use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::test_be_integer;
use crate::code::testing::test_compiler::test_compilation;
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

empty: (
        "{};1;",
        vec![
            make(OpCodes::Hash.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1)]
    ),

a_map: (
    "{1:1,2:2,3:3}",
    vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Constant.into(), &[3]),
            make(OpCodes::Constant.into(), &[4]),
            make(OpCodes::Constant.into(), &[5]),
            make(OpCodes::Hash.into(), &[6]),
            make(OpCodes::Pop.into(), &[]),
    ],
    vec![
        test_be_integer(1),
        test_be_integer(1),
        test_be_integer(2),
        test_be_integer(2),
        test_be_integer(3),
        test_be_integer(3),
    ]
    ),

map_with_calculations: (
    "{1:3+4}",
    vec![
    make(OpCodes::Constant.into(), &[0]),
    make(OpCodes::Constant.into(), &[1]),
    make(OpCodes::Constant.into(), &[2]),
    make(OpCodes::Add.into(), &[]),
    make(OpCodes::Hash.into(), &[2]),
    make(OpCodes::Pop.into(), &[])
    ],
    vec![
        test_be_integer(1),
        test_be_integer(3),
        test_be_integer(4)
    ]
),

}
