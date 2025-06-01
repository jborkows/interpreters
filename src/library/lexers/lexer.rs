use std::iter::Peekable;
use std::rc::Rc;
use std::{cmp, collections::VecDeque, env};

use crate::tokens::Token;

use super::{
    dispatch::{dispatch, end_of_line, finish_it},
    parsing_states::LexerState,
};

pub struct Lexer {
    source: VecDeque<Rc<Token>>,
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
            if env::var("DEBUG").is_ok() {
                println!("Processing character: {}, state: {:?}", c, self.state);
            }
            let result = dispatch(self.current_line, self.current_column, c, &self.state);
            self.state = result.0;
            let tokens = result.1;
            for token in tokens {
                self.source.push_back(Rc::new(token));
            }
        }
        let result = end_of_line(&self.state, self.current_line, self.current_column);
        if let Some(s) = result.0 {
            self.state = s;
        }
        let tokens = result.1;
        for token in tokens {
            self.source.push_back(Rc::new(token));
        }
        self.current_column = cmp::max(1, self.current_column);
    }

    pub fn next(&mut self) -> Option<Rc<Token>> {
        let result = self.source.pop_front();
        match result {
            Some(token) => Some(token),
            None => {
                if env::var("DEBUG").is_ok() {
                    println!("Finishing up {:?}", self.state);
                }
                self.finish()
            }
        }
    }

    fn finish(&mut self) -> Option<Rc<Token>> {
        if env::var("DEBUG").is_ok() {
            println!("Finishing up {:?}", self.state);
        }
        let token = finish_it(&self.state, self.current_line, self.current_column);
        self.state = LexerState::Idle;
        token.map(Rc::new)
    }
}

impl Iterator for Lexer {
    type Item = Rc<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

pub struct LexingIterator<I: Iterator<Item = String>> {
    lines: Peekable<I>,
    lexer: Lexer,
}

impl<I: Iterator<Item = String>> Iterator for LexingIterator<I> {
    type Item = Rc<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try getting next token from current lexer state
            if let Some(token) = self.lexer.next() {
                return Some(token);
            }

            // If no more tokens, try to process next line
            match self.lines.next() {
                Some(line) => {
                    self.lexer.process(&line);
                }
                None => {
                    self.lexer.finish();
                }
            }
        }
    }
}
