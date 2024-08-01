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


use std::iter::Peekable;
use std::str::Chars;

use crate::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { 
            input: input.chars().peekable(),
        }
    }

    fn read_identifier(&mut self, first_char: char) -> String {
        let mut ident = String::new();
       

        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        ident
    }
    fn read_function(&mut self) -> Token {
        let mut ident = String::new();
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(&c) = self.input.peek() {
            if c == '(' {
                self.input.next();
                break;
            } else {
                ident.push(c);
                self.input.next();
            }
        }

        while let Some(&c) = self.input.peek() {
            if c == ')' {
                self.input.next();
                break;
            } else {
                tokens.push(self.next().unwrap());
            }
        }

        Token::Function(ident, tokens)
    }
    fn read_number(&mut self, first_char: char) -> Token {
        let mut num_str = String::new();


        while let Some(&c) = self.input.peek() {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        if num_str.contains('.') {
            println!("float");
            Token::Empty
            
        } else {
            Token::Integer(num_str.parse().unwrap())
        }
    }   

    fn read_string(&mut self) -> String {
        let mut str_literal = String::new();
        self.input.next();

        while let Some(c) = self.input.next() {
            if c == '"' {
                break;
            }
            else if c == '\\' {
                if let Some(c) = self.input.next() {
                    match c {
                        'n' => str_literal.push('\n'),
                        't' => str_literal.push('\t'),
                        '\\' => str_literal.push('\\'),
                        '"' => str_literal.push('"'),
                        _ => str_literal.push(c),
                    }
                }
            } 
            else {
                str_literal.push(c);
            }
        }

        str_literal
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        

        while let Some(&c) = self.input.peek() {
            if c == '\n' {
                self.input.next();
                //return Some(Token::NewLine);
            }
            if c == '\r' {
                self.input.next();
                return Some(Token::NewLine);
            }
            if c == '\t' {
                self.input.next();
                return Some(Token::NewLine);
            }


            match c {
                ' ' | '\n' | '\t' => {
                    self.input.next();
                }
                '0'..='9' => {
                    return Some(self.read_number(c));
                }
                '"' => {
                    return Some(Token::String(self.read_string()));
                }
                '+' => {
                    self.input.next();
                    return Some(Token::Plus);
                }
                '-' => {
                    self.input.next();
                    return Some(Token::Minus);
                }
                '*' => {
                    self.input.next();
                    return Some(Token::Multiply);
                }
                '/' => {
                    self.input.next();
                    return Some(Token::Divide);
                }
                '%' => {
                    self.input.next();
                    return Some(Token::Modulo);
                }
                '=' => {
                    self.input.next();
                    if let Some(&'=') = self.input.peek() {
                        self.input.next();
                        return Some(Token::Equals);
                    } else {
                        return Some(Token::Equals);
                    }
                }
                '!' => {
                    self.input.next();
                    if let Some(&'=') = self.input.peek() {
                        self.input.next();
                        return Some(Token::NotEquals);
                    } else {
                        return Some(Token::Not);
                    }
                }
                '<' => {
                    self.input.next();
                    if let Some(&'=') = self.input.peek() {
                        self.input.next();
                        return Some(Token::LessThanOrEqual);
                    } else {
                        return Some(Token::LessThan);
                    }
                }
                '>' => {
                    self.input.next();
                    if let Some(&'=') = self.input.peek() {
                        self.input.next();
                        return Some(Token::GreaterThanOrEqual);
                    } else {
                        return Some(Token::GreaterThan);
                    }
                }
                '&' => {
                    self.input.next();
                    if let Some(&'&') = self.input.peek() {
                        self.input.next();
                        return Some(Token::And);
                    } else {
                        return None;
                    }
                }
                '|' => {
                    self.input.next();
                    if let Some(&'|') = self.input.peek() {
                        self.input.next();
                        return Some(Token::Or);
                    } else {
                        return None;
                    }
                }
                '(' => {
                    self.input.next();
                    return Some(Token::LeftParen);
                }
                ')' => {
                    self.input.next();
                    return Some(Token::RightParen);
                }
                '{' => {
                    self.input.next();
                    return Some(Token::LeftBrace);
                }
                '}' => {
                    self.input.next();
                    return Some(Token::RightBrace);
                }
                ':' => {
                    self.input.next();
                    return Some(Token::Colon);
                }
                ';' => {
                    self.input.next();
                    return Some(Token::Semicolon);
                }
                ',' => {
                    self.input.next();
                    return Some(Token::Comma);
                }
                _ => {
                    if c.is_alphabetic() {
                        let ident = self.read_identifier(c);
                        match ident.as_str() {
                            "int" => return Some(Token::Int),
                            "float" => return Some(Token::Float),
                            "str" => return Some(Token::Str),
                            "bool" => return Some(Token::Bool),
                            "void" => return Some(Token::Void),
                            "if" => return Some(Token::If),
                            "else" => return Some(Token::Else),
                            "while" => return Some(Token::While),
                            "for" => return Some(Token::For),
                            "print" => {
                                self.input.next();
                                return Some(Token::Print)
                            
                            }
                            _ => return Some(Token::Identifier(ident)),
                        }
                    } else {
                        return None;
                    }
                }
            }
        }

        Some (Token::Eof)
    }
}



