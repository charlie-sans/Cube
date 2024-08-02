// parser.rs
// takes in Vec<Token> and spits out Vec<ParsedToken> where the Parsed tokens are the parsed version of the tokens
// say a:int = 3; turns into ParsedToken::Assignment(Identifier("a"), Token::Int(3))
use std::iter::Peekable;
use std::str::Chars;


use crate::token::*;

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { 
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
            Token::Integer(num_str.parse().unwrap())
        }
    
    fn read_string(&mut self, first_char: char) -> Token {
        let mut string = String::new();
        while let Some(&c) = self.input.peek() {
            if c == '"' {
                self.input.next();
                break;
            } else {
                string.push(c);
                self.input.next();
            }
        }
        Token::String(string)
    }
    fn read_symbol(&mut self, first_char: char) -> Token {
        match first_char {
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '\n' => Token::NewLine,
            ',' => Token::Comma,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            _ => Token::Empty,
        }
    }
    fn read_operator(&mut self, first_char: char) -> Token {
        match first_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '%' => Token::Modulo,
            '=' => Token::Equals,
            '!' => Token::Not,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '&' => Token::And,
            '|' => Token::Or,
            _ => Token::Empty,
        }
    }

    fn read_keyword(&mut self, first_char: char) -> Token {
        let keyword = self.read_identifier(first_char);
        match keyword.as_str() {
            "int" => Token::Int,
            "str" => Token::Str,
            "float" => Token::Float,
            "bool" => Token::Bool,
            "void" => Token::Void,
            _ => Token::Identifier(keyword),
        }
    }

    fn read_token(&mut self, first_char: char) -> Token {
        match first_char {
            '0'..='9' => self.read_number(first_char),
            'a'..='z' | 'A'..='Z' => self.read_keyword(first_char),
            '"' => self.read_string(first_char),
            ':' | ';' | '\n' | ',' | '(' | ')' | '{' | '}' => self.read_symbol(first_char),
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' => self.read_operator(first_char),
            _ => Token::Empty,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        match self.input.peek() {
            Some(&c) => Some(self.read_token(c)),
            None => None,
        }
    }
}