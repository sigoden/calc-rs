mod eval;
mod lex;
mod parser;

use std::io::{self, Write};

fn main() {
    println!("Welcome to calc!");
    loop {
        print!(">>> ");
        if let Err(x) = io::stdout().flush() {
            println!("can not flush: {}", x);
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("can not read line.");
        let toks = lex::lex(&input);
        let mut parser = parser::Parser::new(toks);
        match parser.parse() {
            Ok(expr) => {
                println!("{}", eval::eval(&expr));
            }
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }
}
