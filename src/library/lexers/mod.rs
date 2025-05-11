mod dispatch;
mod idle;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parsers;
mod parsing_states;
mod reading_equality;
mod reading_invalid;
mod reading_minus;
mod reading_negation;
mod reading_number;

pub use lexer::Lexer;
