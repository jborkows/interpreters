mod allocator;
mod ast;
mod code;
mod evaluator;
mod lexers;
mod lines;
mod object;
mod parser;
mod processors;
mod repl;
mod tokens;
mod vm;

#[macro_export]
macro_rules! print_bash_error {
    ($msg:expr) => {
        format!("\x1b[31m{}\x1b[0m", $msg)
    };
}

pub use repl::start;

pub fn bar() {
    println!("Hello, world!");
}
pub use processors::Compiler;
