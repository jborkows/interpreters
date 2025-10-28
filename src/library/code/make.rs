use crate::code::{
    definitions::{Byte, Definition, Instructions, OpCode},
    lookup, read_u_16,
};

pub(crate) fn make(opcode: OpCode, operands: &[u16]) -> Instructions {
    let maybe_definition = lookup(&opcode);
    let definition = match maybe_definition {
        Ok(v) => v,
        //TODO more useful message
        Err(_) => panic!("Cannot find definition for {opcode:?}"),
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

pub(crate) fn read_operands(definition: &Definition, operands: &[Byte]) -> (Vec<u16>, usize) {
    let mut values: Vec<u16> = vec![0; definition.operands_widths.len()];
    let mut offset: usize = 0;
    for (index, operand_width) in definition.operands_widths.iter().enumerate() {
        match operand_width {
            2 => {
                let value: u16 = read_u_16(&operands[offset..]);
                values[index] = value;
                offset += operand_width;
            }
            _ => todo!(),
        }
    }

    (values, offset)
}

fn convert(incoming: Vec<u8>) -> Instructions {
    let bytes: Vec<Byte> = incoming.iter().map(|v| v.into()).collect();
    return Instructions(bytes);
}
