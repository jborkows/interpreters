use std::io::{self, BufRead};

use crate::{
    evaluator::{define_macros, expand_macros},
    lexers::Lexer,
    object::new_environment,
    parser::Parser,
};

pub fn start() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let environemnt = new_environment();
    let macro_environemnt = new_environment();
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

        println!("Parsed program: {}", program);
        let macro_defined = define_macros(program, macro_environemnt.clone());
        let macro_expanded_program = expand_macros(macro_defined, macro_environemnt.clone());

        println!("###########");
        println!("After macro expansion program: {}", macro_expanded_program);
        let result = crate::evaluator::evaluate(&macro_expanded_program, environemnt.clone());
        println!("Evaluation result: {}", result);
    }
}
