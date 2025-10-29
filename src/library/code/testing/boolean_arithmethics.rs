use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::{test_be_integer, test_compilation};
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

true_conversion: (
        "true;1",
        vec![
            make(OpCodes::True.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],

            vec![test_be_integer(1)]
    ),
false_conversion: (
        "false;1",
        vec![
            make(OpCodes::False.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],

            vec![test_be_integer(1)] //I don't know how to satisfy empty array...
    ),
greater_than: (
        "1 > 2",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::GreaterThan.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],

            vec![test_be_integer(1),test_be_integer(2)]
    ),

less_than: (
        "1 < 2",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::GreaterThan.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
            vec![test_be_integer(2),test_be_integer(1)]
    ),

equal: (
        "1 == 2",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Equal.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
            vec![test_be_integer(1),test_be_integer(2)]
    ),
not_equal: (
        "1 != 2",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::NotEqual.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
            vec![test_be_integer(1),test_be_integer(2)]
    ),

bang_false: (
        "!false;1",
        vec![
            make(OpCodes::False.into(), &[]),
            make(OpCodes::Bang.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],

            vec![test_be_integer(1)] //I don't know how to satisfy empty array...
    ),

bang_true: (
        "!true;1",
        vec![
            make(OpCodes::True.into(), &[]),
            make(OpCodes::Bang.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],

            vec![test_be_integer(1)] //I don't know how to satisfy empty array...
    ),
}
