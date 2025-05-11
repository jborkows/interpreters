mod dispatch;
mod idle;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parsers;
mod parsing_states;
mod reading_equality;
mod reading_negation;

pub use lexer::Lexer;
