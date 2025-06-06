use std::{env, fmt::Display, ops::Add};

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TextPosition {
    pub line_number: LineNumber,
    pub column_number: ColumnNumber,
}

impl TextPosition {
    pub fn new(line_number: u16, column_number: u16) -> Self {
        Self {
            line_number: LineNumber(line_number),
            column_number: ColumnNumber(column_number),
        }
    }

    pub fn token_ends_with(&self, line_number: u16, column_number: u16) -> TokenPosition {
        TokenPosition::new(*self, TextPosition::new(line_number, column_number))
    }
}
impl Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if env::var("DEBUG").is_ok() {
            write!(
                f,
                "Line: {}, Column: {}",
                self.line_number.0, self.column_number.0
            )
        } else {
            write!(f, "{},{}", self.line_number.0, self.column_number.0)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TokenPosition {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TokenPosition {
    pub fn new(start: TextPosition, end: TextPosition) -> Self {
        Self { start, end }
    }
    pub fn single_character(line_number: LineNumber, column_number: ColumnNumber) -> Self {
        Self {
            start: TextPosition {
                line_number,
                column_number,
            },
            end: TextPosition {
                line_number,
                column_number,
            },
        }
    }
    fn is_single_character(&self) -> bool {
        self.start.line_number == self.end.line_number
            && self.start.column_number == self.end.column_number
    }
}
impl Display for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_single_character() {
            write!(
                f,
                "({},{})",
                self.start.line_number.0, self.start.column_number.0
            )
        } else {
            write!(f, "({})->({})", self.start, self.end)
        }
    }
}
