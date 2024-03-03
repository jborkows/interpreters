mod base;
mod lexer;
mod processors;
mod read_idle;
mod read_number;
mod read_operator;
mod read_text;
mod tokens;
pub use base::{ColumnNumber, LineNumber, SourceCharecter};
pub use lexer::read_all;
pub use tokens::{Token, TokenKind};

#[cfg(test)]
mod lexer_tests;
