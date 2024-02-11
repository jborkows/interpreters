use std::borrow::BorrowMut;
use std::io;

use interpreter::bar;
use interpreter::start;

fn main() {
    println!("Enter text (Ctrl+D to end):");
    start(io::stdin().borrow_mut(), io::stdout().borrow_mut());
}
