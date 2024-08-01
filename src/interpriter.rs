use std::iter::Peekable;
use std::str::Chars;


use crate::Token;

fn print_f(msg: &str) {
    println!("{}", msg);
}
//𓎢𓄿𓈖 𓇌𓅲 𓅲𓈖𓂧𓂋𓋴𓏏𓄿𓈖𓂧 𓅓?
pub fn interpret(tokens: Vec<Token>) {
   let mut i = 0;
    while i < tokens.len() {
         match tokens[i] {
                Token::NewLine => {
                    i += 1;
                },
                Token::Print => {
                i += 1;
                match tokens[i] {
                     Token::String(ref msg) => {
                          print_f(msg);
                     },
                     Token::Identifier(ref ident) => {
                          print_f(ident);
                     },
                    
                     _ => {
                          panic!("Expected a string or identifier after print");
                     }
                     // should end with a right parenthesis
                     Token::RightParen => {
                          i += 1;
                     },
                }
                i += 1;
              },
              _ => {
                panic!("Unexpected token {:?}", tokens[i]);
              }
         }
         i += 1;
    }
        
    
}