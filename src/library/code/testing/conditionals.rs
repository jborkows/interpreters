use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::{test_be_integer, test_compilation};
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

single_if: (
        "if(true) {10}; 3333",
        vec![
            make(OpCodes::True.into(), &[]), //0000
            make(OpCodes::JumpNotTruthy.into(), &[10]), //0001 -> jump outside condition if false, so move
            //to stack position where if is not more -> 3333;
            make(OpCodes::Constant.into(), &[0]), //0004
            make(OpCodes::Jump.into(), &[11]), //0007
            make(OpCodes::Null.into(), &[]), //0010
            make(OpCodes::Pop.into(), &[]),//0011
            make(OpCodes::Constant.into(), &[1]),//0012
            make(OpCodes::Pop.into(), &[]), //0015
        ],
        vec![test_be_integer(10),test_be_integer(3333)]
    ),
if_else: (
        "if(true) {10} else {30}; 3333",
        vec![
            make(OpCodes::True.into(), &[]), //0000
            make(OpCodes::JumpNotTruthy.into(), &[10]), //0001 -> jump outside condition if false, so move
            //to stack position where if is not more -> 3333;
            make(OpCodes::Constant.into(), &[0]), //0004
            make(OpCodes::Jump.into(), &[13]), //0007
            make(OpCodes::Constant.into(), &[1]), //0010
            make(OpCodes::Pop.into(), &[]),//0013
            make(OpCodes::Constant.into(), &[2]),//0014
            make(OpCodes::Pop.into(), &[]), //0017
        ],
        /**
        * 0000 True
        * 0001 JumpNotTruthy
        * 0002 0
        * 0003 10
        * 0004 Constant
        * 0005 0
        * 0006 0
        * 0007 Jump
        * 0008 0
        * 0009 13
        * 0010 Constant
        * 0011 0
        * 0012 1
        * 0013 Pop
        * 0014 Constant
        * 0015 0
        * 0016 2
        * 0017 Pop
        *
        */
        vec![test_be_integer(10), test_be_integer(30),test_be_integer(3333)]
    ),
}
