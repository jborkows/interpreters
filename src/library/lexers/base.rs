#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineNumber(pub u16);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColumnNumber(pub u16);

pub trait Lexable {
    fn next_line(&self) -> Option<(LineNumber, String)>;
}
