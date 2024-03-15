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

impl From<usize> for LineNumber {
    fn from(val: usize) -> Self {
        LineNumber(val as u16)
    }
}
impl From<usize> for ColumnNumber {
    fn from(val: usize) -> Self {
        ColumnNumber(val as u16)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SourceCharecter {
    pub(crate) ch: char,
    pub column_number: ColumnNumber,
    pub line_number: LineNumber,
}

impl SourceCharecter {
    pub fn new(ch: char, column_number: ColumnNumber, line_number: LineNumber) -> Self {
        Self {
            ch,
            column_number,
            line_number,
        }
    }
    pub fn is_whitespace(&self) -> bool {
        self.ch.is_whitespace()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct StateLineContext {
    pub(crate) text: String,
    pub(crate) line: LineNumber,
    pub(crate) column: ColumnNumber,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum State {
    Idle,
    ReadingText(StateLineContext),
    ReadingNumber(StateLineContext),
    ReadingOperator(StateLineContext),
}
