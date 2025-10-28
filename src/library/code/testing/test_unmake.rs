use std::vec;

use indoc::indoc;

use crate::code::{
    definitions::{OpCode, OpCodes},
    lookup,
    make::{make, read_operands},
    testing::test_compiler::concat_instructions,
};

#[test]
fn should_display_opcodes_for_constants() {
    let instructions = vec![
        make(OpCodes::Constant.into(), &[1]),
        make(OpCodes::Constant.into(), &[2]),
        make(OpCodes::Constant.into(), &[65535]),
    ];
    let expected = indoc! {"
        0x0000 Constant 1
        0x0003 Constant 2
        0x0006 Constant 65535
    "};
    let flatten = concat_instructions(&instructions);
    assert_eq!(expected, flatten.to_string())
}

macro_rules! generate_for_display {

    ($($name:ident: ($op_codes:expr,$operands:expr, $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let instruction = make($op_codes.into(), &$operands);
                assert_eq!($expected.to_string()+"\n", instruction.to_string())
            }
        )*
    };
}

generate_for_display! {
   should_display_add: (OpCodes::Add, [], "0x0000 +"),
}

fn helper_read(op_codes: OpCodes, operands: &[u16], read_bytes: usize) {
    let op_code: OpCode = op_codes.into();
    let instructions = make(op_code.clone(), operands);
    let definition = match lookup(&op_code) {
        Ok(v) => v,
        Err(e) => panic!(
            "Cannot find definition for {:?} ended with {:?}",
            &op_code, e
        ),
    };
    let (operands_read, number_of_bytes) = read_operands(definition, &instructions.bytes()[1..]);
    assert_eq!(
        number_of_bytes, read_bytes,
        "Should read {:?} but got {:?}",
        read_bytes, number_of_bytes
    );
    for (index, value) in operands_read.iter().zip(operands).enumerate() {
        let (read, expected) = value;
        assert_eq!(
            read, expected,
            "Expected to read {:?} but was {:?} at {:?}",
            expected, read, index
        )
    }
}
macro_rules! generate_for_read {

    ($($name:ident: ($op_codes:expr,$operands:expr, $to_read_bytes:expr),)*) => {
        $(
            #[test]
            fn $name() {
                helper_read($op_codes,$operands,$to_read_bytes );
            }
        )*
    };

}

generate_for_read! {
   should_read_constant: (OpCodes::Constant, &[65535], 2),
}
