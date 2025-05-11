use std::{cmp, collections::VecDeque};

use crate::tokens::Token;

use super::{
    dispatch::{dispatch, finish_it},
    parsing_states::LexerState,
};

pub struct Lexer {
    source: VecDeque<Token>,
    current_line: u16,
    current_column: u16,
    state: LexerState,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            source: VecDeque::new(),
            current_line: 0,
            current_column: 0,
            state: LexerState::Idle,
        }
    }

    pub fn process(&mut self, line: &str) {
        self.current_line += 1;
        self.current_column = 0;
        for (i, c) in line.chars().enumerate() {
            self.current_column = i as u16 + 1;
            println!("Processing character: {}, state: {:?}", c, self.state);
            let result = dispatch(self.current_line, self.current_column, c, &self.state);
            self.state = result.0;
            let tokens = result.1;
            for token in tokens {
                self.source.push_back(token);
            }
        }
        self.current_column = cmp::max(1, self.current_column);
    }

    pub fn peek(&self) -> Option<&Token> {
        self.source.front()
    }

    pub fn next(&mut self) -> Option<Token> {
        let result = self.source.pop_front();
        match result {
            Some(token) => {
                return Some(token);
            }
            None => {
                println!("Finishing up {:?}", self.state);
                let token = finish_it(&self.state, self.current_line, self.current_column);
                self.state = LexerState::Idle;
                return token;
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
