// lexer.rs for cube
// also acts as tokeniser

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Token, is_keyword, ASTNode};

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


pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() {
                ident.push(c);
                self.consume();
            } else {
                break;
            }
        }
        ident
    }

    fn consume_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.consume();
            } else {
                break;
            }
        }
        num_str.parse().unwrap()
    }

    fn consume_string(&mut self) -> String {
        let mut s = String::new();
        self.consume(); // consume the opening quote
        while let Some(&c) = self.peek() {
            if c == '"' {
                self.consume();
                break;
            } else {
                s.push(c);
                self.consume();
            }
        }
        s
    }

    fn consume_boolean(&mut self) -> bool {
        let mut b = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphabetic() {
                b.push(c);
                self.consume();
            } else {
                break;
            }
        }
        match b.as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("Expected boolean"),
        }
    }

    fn consume_keyword(&mut self) -> String {
        let mut kw = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphabetic() {
                kw.push(c);
                self.consume();
            } else {
                break;
            }
        }
        kw
    }

    fn consume_operator(&mut self) -> String {
        let mut op = String::new();
        while let Some(&c) = self.peek() {
            if c.is_ascii_punctuation() {
                op.push(c);
                self.consume();
            } else {
                break;
            }
        }
        op
    }

    fn consume_type(&mut self) -> String {
        let mut ty = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphabetic() {
                ty.push(c);
                self.consume();
            } else {
                break;
            }
        }
        ty
    }

    fn consume_int(&mut self) -> i32 {
        let mut num_str = String::new();
        while let Some(&c) = self.peek() {
            if c.is_numeric() {
                num_str.push(c);
                self.consume();
            } else {
                break;
            }
        }
        num_str.parse().unwrap()
    }

    fn consume_float(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.consume();
            } else {
                break;
            }
        }
        num_str.parse().unwrap()
    }

    fn consume_void(&mut self) {
        let void = "void";
        for c in void.chars() {
            match self.peek() {
                Some(&ch) if ch == c => {
                    self.consume();
                }
                _ => panic!("Expected void"),
            }
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.peek() {
            match c {
                ' ' => {
                    self.consume();
                }
                ':' => {
                    self.consume();
                    tokens.push(Token::Operator(":".to_string()));
                }
                '=' => {
                    self.consume();
                    tokens.push(Token::Operator("=".to_string()));
                }
                '(' => {
                    self.consume();
                    tokens.push(Token::Operator("(".to_string()));
                }
                ')' => {
                    self.consume();
                    tokens.push(Token::Operator(")".to_string()));
                }
                '{' => {
                    self.consume();
                    tokens.push(Token::Operator("{".to_string()));
                }
                '}' => {
                    self.consume();
                    tokens.push(Token::Operator("}".to_string()));
                }
                ';' => {
                    self.consume();
                    tokens.push(Token::Operator(";".to_string()));
                }
                ',' => {
                    self.consume();
                    tokens.push(Token::Operator(",".to_string()));
                }
                '+' => {
                    self.consume();
                    tokens.push(Token::Operator("+".to_string()));
                }
                '-' => {
                    self.consume();
                    tokens.push(Token::Operator("-".to_string()));
                }
                '*' => {
                    self.consume();
                    tokens.push(Token::Operator("*".to_string()));
                }
                '/' => {
                    self.consume();
                    tokens.push(Token::Operator("/".to_string()));
                }
                '<' => {
                    self.consume();
                    tokens.push(Token::Operator("<".to_string()));
                }
                '>' => {
                    self.consume();
                    tokens.push(Token::Operator(">".to_string()));
                }
                '!' => {
                    self.consume();
                    tokens.push(Token::Operator("!".to_string()));
                }
                '&' => {
                    self.consume();
                    tokens.push(Token::Operator("&".to_string()));
                }
                '|' => {
                    self.consume();
                    tokens.push(Token::Operator("|".to_string()));
                }
                '"' => {
                    tokens.push(Token::String(self.consume_string()));
                }
                _ => {
                    if c.is_alphabetic() {
                        let ident = self.consume_identifier();
                        if is_keyword(&ident) {
                            tokens.push(Token::Keyword(ident));
                        } else {
                            tokens.push(Token::Identifier(ident));
                        }
                    } else if c.is_numeric() {
                        let num = self.consume_number();
                        if num as f64 == num {
                            tokens.push(Token::Int(num as i32));
                        } else {
                            tokens.push(Token::Float(num));
                        }
                    } else {
                        self.consume();
                    }

                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}

pub fn interpret(ast: &ASTNode) {
    match ast {
        ASTNode::Function(ident, nodes) => {
            match ident.as_str() {
                "main" => {
                    println!("fn main() {{");
                    for node in nodes {
                        interpret(node);
                    }
                    println!("}}");
                }
                "print" => {
                    println!("println!({});", nodes[0]);
                }
                "if" => {
                    println!("if {} {{", nodes[0]);
                    interpret(&nodes[1]);
                    println!("}} else {{");
                    interpret(&nodes[2]);
                    println!("}}");
                }
                "for" => {
                    println!("for {} in {}..{} {{", nodes[0], nodes[1], nodes[2]);
                    interpret(&nodes[3]);
                    println!("}}");
                }
                "while" => {
                    println!("while {} {{", nodes[0]);
                    interpret(&nodes[1]);
                    println!("}}");
                }
                "return" => {
                    println!("return {};", nodes[0]);
                }
                _ => panic!("Unknown function"),
            }
        }
        ASTNode::Number(num) => {
            println!("{}", num);
        }
        ASTNode::Identifier(ident) => {
            println!("{}", ident);
        }
        ASTNode::Operator(op) => {
            println!("{}", op);
        }
        ASTNode::Keyword(kw) => {
            println!("{}", kw);
        }
        ASTNode::String(s) => {
            println!("{}", s);
        }
        ASTNode::Boolean(b) => {
            println!("{}", b);
        }
        ASTNode::Int(i) => {
            println!("{}", i);
        }
        ASTNode::Float(fl) => {
            println!("{}", fl);
        }
        ASTNode::Void => {
            println!("void");
        }
        ASTNode::SemiColon => {
            println!(";");
        }
        ASTNode::EOF => {}
    }
}


