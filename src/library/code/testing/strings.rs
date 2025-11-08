use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::{test_be_string, test_compilation};
use crate::generate_tests_for_compiler;

generate_tests_for_compiler! {

define_string: (
        r#"
        "AAA"
        "#,
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_string("AAA")]
    ),

string_addint: (
    r#"
    "A" + "B"
    "#,
    vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
    ],
    vec![test_be_string("A"), test_be_string("B")]
    ),

}
