// cube, a programming language that is inspired by Rust and Python

mod lexer;
mod parser;
mod interpriter;
pub mod token;

use std::io::{self, Write};
use std::fs::*;
use interpriter::interpret;
use lexer::{Lexer};
use crate::token::Token;
fn print_s(msg: &str, i: i32) {
    println!("{}{}", msg, i);
}
fn print_s_token(msg: &str, token: Token) {
    println!("{}{:?}", msg, token);
}
fn main() {
    let mut input = String::new();
    let mut file = String::new();
    print!("cube> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    // read the file contents
    if input.trim() == "run" {
        // read the next argument as the file name in the directory relative to the current directory
        let mut file_name = String::new();
        print!("file> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut file_name).unwrap();
        file = read_to_string(file_name.trim()).unwrap();
    } else {
        file = input;
    }
    // create a lexer
    let lexer = Lexer::new(&file);
    let mut tokens: Vec<Token> = Vec::new(); // create a vector to store the tokens
    println!("these are our tokens {:?}", lexer);
    for token in lexer { 
        if &token == &Token::Eof {
            println!("EOF");
            break;
        }
            //print("cat"); print("catss");
       
            tokens.push(token);
    
    }
    //println!("these are our tokens{:?}", tokens);
    
    interpret(tokens);

   
}