mod lexers;
mod lines;
mod repl;
mod tokens;

pub use repl::start;

pub fn bar() {
    println!("Hello, world!");
}
