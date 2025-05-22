mod dispatch;
mod idle;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parsers;
mod parsing_states;
mod reading_equality;
mod reading_identifier;
mod reading_invalid;
mod reading_negation;
mod reading_number;
mod reading_text;

pub use lexer::Lexer;
