use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Byte(pub u8);

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for Byte {
    fn from(val: u8) -> Self {
        Byte(val)
    }
}

impl From<Byte> for u8 {
    fn from(val: Byte) -> Self {
        val.0
    }
}

impl From<&u8> for Byte {
    fn from(val: &u8) -> Self {
        Byte(*val)
    }
}

pub struct Instructions(pub Vec<Byte>);

impl From<Vec<Byte>> for Instructions {
    fn from(value: Vec<Byte>) -> Self {
        Instructions(value)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct OpCode(pub Byte);

pub struct Definition {
    pub name: String,
    pub operands_widths: Vec<usize>, //can have multiple operands with different width
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value.0.into()
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum OpCodes {
    Constant,
}

impl From<OpCodes> for OpCode {
    fn from(value: OpCodes) -> Self {
        OpCode(Byte(value as u8))
    }
}

impl From<OpCodes> for u8 {
    fn from(value: OpCodes) -> Self {
        let op_code: OpCode = value.into();
        op_code.into()
    }
}
