use crate::code::{definitions::OpCodes, make::make};

fn compare_bytecode(incoming: OpCodes, incoming_operands: &[u16], expected_bytes: &[u8]) {
    let instructions = make(incoming.into(), &incoming_operands).0;
    assert_eq!(
        instructions.len(),
        expected_bytes.len(),
        "instructions are different lenght than expected. Expecting {}, got {}",
        expected_bytes.len(),
        instructions.len(),
    );

    for (index, (i, e)) in instructions.iter().zip(expected_bytes.iter()).enumerate() {
        assert_eq!(
            i,
            &e.into(),
            "instruction at {} -> {} should equal {}",
            index,
            i,
            e
        )
    }
}

macro_rules! expect_comparing {
    ($($name:ident: ($input:expr,$operands:expr,  $expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                 compare_bytecode($input, $operands, $expected);
            }
        )*
    };
}

expect_comparing! {
        should_create_constant: (OpCodes::Constant, &[65534], &[OpCodes::Constant.into(), 255,254]),
        should_create_add: (OpCodes::Add, &[], &[OpCodes::Add.into()]),
}
