use std::collections::VecDeque;

use crate::tokens::Token;

use super::{dispatch::dispatch, parsing_states::LexerState};

pub struct Lexer {
    source: VecDeque<Token>,
    current_line: u16,
    state: LexerState,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            source: VecDeque::new(),
            current_line: 0,
            state: LexerState::Idle,
        }
    }

    pub fn process(&mut self, line: &str) {
        self.current_line += 1;
        for (i, c) in line.chars().enumerate() {
            let column = i as u16 + 1;
            let result = dispatch(self.current_line, column, c, &self.state);
            self.state = result.0;
            let tokens = result.1;
            for token in tokens {
                self.source.push_back(token);
            }
        }
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
