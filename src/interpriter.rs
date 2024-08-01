use crate::{token, Token};
use crate::token::ASTNode;
use std::fmt::format;
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;
use regex::Regex;
use std::collections::hash_map::Entry;


fn print_f(msg: &str) {
    println!("{}", msg);
}

/*

main:void()
{
a:int = 5;
print(a);
}


*/


pub fn interpret(tokens: Vec<ASTNode>) {
    let mut i = 0;
    let mut variables: HashMap<String, Token> = HashMap::new();
    let mut objects: HashMap<String, Token> = HashMap::new();
    let mut functions: HashMap<String, Token> = HashMap::new();
    let mut classes: HashMap<String, Token> = HashMap::new();
    let mut function_args: HashMap<String, Token> = HashMap::new();
    let mut class_args: HashMap<String, Token> = HashMap::new();
    let mut class_methods: HashMap<String, Token> = HashMap::new();
    let mut class_variables: HashMap<String, Token> = HashMap::new();

    while i < tokens.len() {
        match &tokens[i] {
            ASTNode::Function(name, args, body) => {
                functions.insert(name.to_string(), Token::Function(name.to_string(), args.to_vec(), body.to_vec()));
            }
            _ => {
                print_f("Error: Invalid token");
            }
        }
        i += 1;
    }

 
    println!("{:?}", variables);
    println!("{:?}", objects);
    println!("{:?}", functions);
    println!("{:?}", classes);
    println!("{:?}", function_args);
}
