use crate::code::{definitions::Byte, definitions::Instructions, definitions::OpCode, lookup};

pub(crate) fn make(opcode: OpCode, operands: Vec<u16>) -> Instructions {
    let maybe_definition = lookup(&opcode);
    let definition = match maybe_definition {
        Ok(v) => v,
        //TODO more useful message
        Err(_) => return Instructions(vec![]),
    };
    let mut instruction_size = 1;
    for val in &definition.operands_widths {
        instruction_size += val;
    }

    let mut instructions: Vec<u8> = vec![0; instruction_size];
    instructions[0] = opcode.into();
    let mut offset = 1;
    for (i, val) in operands.iter().enumerate() {
        let current_width = definition.operands_widths[i];
        match current_width {
            2 => {
                let big_endian = val.to_be_bytes();
                instructions[offset] = big_endian[0];
                instructions[offset + 1] = big_endian[1];
            }
            _ => {}
        }
        offset += current_width
    }
    return convert(instructions);
}

fn convert(incoming: Vec<u8>) -> Instructions {
    let bytes: Vec<Byte> = incoming.iter().map(|v| v.into()).collect();
    return Instructions(bytes);
}
