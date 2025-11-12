use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::{test_be_integer, test_compilation};
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

two_assigments: (
        "
        let x = 1;
        let y = 2;
        ",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::SetGlobal.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::SetGlobal.into(), &[1]),
        ],
        vec![test_be_integer(1),test_be_integer(2)]
    ),
single_expression: (
        "
        let x = 1;
        x;
        ",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::SetGlobal.into(), &[0]),
            make(OpCodes::GetGlobal.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1)]

),
reassing: (
        "
        let x = 1;
        let y = x;
        y;
        ",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::SetGlobal.into(), &[0]),
            make(OpCodes::GetGlobal.into(), &[0]),
            make(OpCodes::SetGlobal.into(), &[1]),
            make(OpCodes::GetGlobal.into(), &[1]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1)]

),

}
