use std::io::{self, BufRead};

use crate::{lexers::Lexer, parser::Parser};

pub fn start() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let mut lexer = Lexer::new();
        let line = line_result.unwrap();
        lexer.process(&line);

        let program = Parser::from_string(line.as_str()).parse_program();

        println!(
            "You entered: {}",
            lexer
                .into_iter()
                .map(|token| format!("{:?}", token.as_ref().kind))
                .collect::<String>()
        );
        println!("Parsed program: {:?}", program.statements.iter().count());
    }
}
