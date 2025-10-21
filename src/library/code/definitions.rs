use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Byte(pub u8);

impl Byte {
    fn string(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}", self.0)
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string(f)
    }
}
impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string(f)
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

impl Instructions {
    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn bytes(&self) -> Vec<Byte> {
        self.0.clone()
    }
}

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
