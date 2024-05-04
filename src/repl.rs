use std::io::{self, Write};

use crate::lexer;
use crate::token;

pub fn start() {
    let mut buffer: String;
    loop {
        buffer = String::new();
        print!(">>>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer);
        let mut l: lexer::Lexer = lexer::Lexer::new(buffer);
        let mut tok = l.next_token();
        while tok.Type != token::EOF {
            println!("{}", tok);
            tok = l.next_token();
        }
    }
}
