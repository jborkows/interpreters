use std::collections::VecDeque;

use crate::tokens::Token;

pub struct Lexer {
    source: VecDeque<Token>,
}

enum LexerState {
    Idle,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            source: VecDeque::new(),
        }
    }

    pub fn process(&mut self, line: &str) {
        panic!("Lexer process not implemented");
    }

    pub fn peek(&self) -> Option<&Token> {
        self.source.front()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.source.pop_front()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
