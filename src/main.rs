use crate::repl::start;

pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello, world!");
    start();
}
