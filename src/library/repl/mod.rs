use std::io::{self, BufRead};

use crate::{lexers::Lexer, parser::Parser};

pub fn start() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut environemnt = crate::object::Environment::new();
    for line_result in reader.lines() {
        let mut lexer = Lexer::new();
        let line = line_result.unwrap();
        lexer.process(&line);
        let mut parser = Parser::from_string(line.as_str());
        let program = parser.parse_program();
        let errors = parser.errors();
        if !errors.is_empty() {
            println!("Errors found in the program:");
            for error in errors {
                println!("{}", error);
            }
            continue;
        }

        println!("Parsed program: {}", program.to_string());
        let result = crate::evaluator::evaluate(&program, &mut environemnt);
        println!("Evaluation result: {}", result.to_string());
    }
}
