mod fake_source;
mod lexers;
mod parser;
mod repl;

pub use repl::start;

pub fn bar() {
    println!("Hello, world!");
}
