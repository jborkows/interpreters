use std::ops::Add;

use super::tokens::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineNumber(pub u16);

impl LineNumber {}
impl Add<u16> for LineNumber {
    type Output = LineNumber;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColumnNumber(pub u16);

impl Add<u16> for ColumnNumber {
    type Output = ColumnNumber;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}
pub trait Collector {
    fn consume(token: Token);
}

pub trait Lexable {
    fn next_line(&self) -> Option<(LineNumber, String)>;
}
