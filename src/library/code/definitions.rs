use std::fmt::{Debug, Display};

use crate::code::{lookup, make::read_operands};

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

#[derive(Clone)]
pub struct Instructions(pub Vec<Byte>);

impl Instructions {
    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn bytes(&self) -> Vec<Byte> {
        self.0.clone()
    }

    fn string(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;

        while index < self.0.len() {
            let definition = lookup(&OpCode(self.0[index].clone())).unwrap_or_else(|e| {
                panic!("While parsing expression at index = {index:#06X}: {e:?}")
            });

            let (operands, read) = read_operands(definition, &self.0[index + 1..]);

            write!(f, "{index:#06X} {}", definition.name)?;

            for operand in &operands {
                write!(f, " {operand}")?;
            }

            writeln!(f)?;
            index += read + 1;
        }

        Ok(())
    }
}
impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string(f)
    }
}
impl Debug for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string(f)
    }
}

impl From<Vec<Byte>> for Instructions {
    fn from(value: Vec<Byte>) -> Self {
        Instructions(value)
    }
}

impl From<Byte> for OpCode {
    fn from(value: Byte) -> Self {
        OpCode(value)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct OpCode(pub Byte);

pub struct Definition {
    pub name: String,
    pub operands_widths: Vec<usize>, //can have multiple operands with different width
}

impl From<&Byte> for u8 {
    fn from(value: &Byte) -> Self {
        value.0
    }
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
    Add,
    Pop,
    Subtitute,
    Multiply,
    Divide,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
    Minus,
    Bang,
    Jump,
    JumpNotTruthy,
    Null,
    SetGlobal,
    GetGlobal,
    Array,
    Hash,
    Index,
}

impl Display for OpCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCodes::Constant => f.write_str("Constant"),
            OpCodes::Add => f.write_str("+"),
            OpCodes::Subtitute => f.write_str("-"),
            OpCodes::Divide => f.write_str("/"),
            OpCodes::Multiply => f.write_str("*"),
            OpCodes::Pop => f.write_str("Pop"),
            OpCodes::True => f.write_str("True"),
            OpCodes::False => f.write_str("False"),
            OpCodes::Equal => f.write_str("=="),
            OpCodes::NotEqual => f.write_str("!="),
            OpCodes::GreaterThan => f.write_str(">"),
            OpCodes::Minus => f.write_str("-"),
            OpCodes::Bang => f.write_str("!"),
            OpCodes::Jump => f.write_str("Unconditional jump"),
            OpCodes::JumpNotTruthy => f.write_str("jump if not True"),
            OpCodes::Null => f.write_str("Null"),
            OpCodes::SetGlobal => f.write_str("SetGlobal"),
            OpCodes::GetGlobal => f.write_str("GetGlobal"),
            OpCodes::Array => f.write_str("[]"),
            OpCodes::Hash => f.write_str("{}"),
            OpCodes::Index => f.write_str("index"),
        }
    }
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
