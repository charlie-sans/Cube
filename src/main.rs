// cube, a programming language that is inspired by Rust and Python

//lexer for the cube programming language

/*  
example

x: int = 5
y:str = "hello"
z:float = 5.5


main:void(){
    print(x);
    print(y);
    print(z);
    for (i: int = 0; i < 10; i++){
        print(i);
        }
        if (x === 5){
            print("x is 5");
            }
            else{
                print("x is not 5");
                }
    while (x < 10){
        print(x);
        x = x + 1;
        }
        
    }
    
    
    
    */
    //mod interpriter;
    mod lexer;
    pub mod token;
    pub mod interpriter;
pub mod parser;


use std::io::{self, Write};
use std::fs::*;


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

    let lex = lexer::Lexer::new(&file);
    let mut tokens = lexer::Lexer::lex(&mut lexer::Lexer::new(&file));
    let mut parser = parser::Parser::new(&tokens);
    let mut ast = parser.parse();
    println!("{:?}", ast);
    // let mut interpriter = interpriter::interpret(&ast);
    // interpriter.interprit();




}