
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;
use std::fs::File;
use std::fs::*;
use std::fmt;

use crate::Token;

pub struct parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> parser<'a> {
    pub fn new(input: &'a str) -> Self {
        parser {
            input: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&c) = self.input.peek() {
            match c {
                ' ' | '\n' => {
                    self.input.next();
                }
                '0'..='9' => {
                    let num = self.parse_number();
                    return Some(Token::Integer(num));
                }
                _ => {
                    self.input.next();
                }
            }
        }
        None
    }
}

impl<'a> parser<'a> {
    fn parse_number(&mut self) -> i32 {
        let mut num_str = String::new();

        while let Some(&c) = self.input.peek() {
            if c.is_numeric() {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        num_str.parse().unwrap()
    }
}


pub fn parse(input: &str) -> Vec<Token> {
    let mut parser = parser::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = parser.next() {
        match token {
            Token::Integer(num) => {
                tokens.push(Token::Integer(num));
            }
            Token::Identifier(ident) => {
                // if the next token is a colon, then it could be a function definition
                if let Some(&c) = parser.input.peek() {
                    if c == ':' {
                        // we might be at a function definition
                        // lets check if we have () before we assume it is a function definition
                        let mut parens = 0;
                        while let Some(&c) = parser.input.peek() {
                            if c == '(' {
                                parens += 1;
                            } else if c == ')' {
                                parens -= 1;
                            }
                            else {
                                // it's not a function definition but a function call
                                //lets cache the identifier and move on
                                break;
                            } 
                            parser.input.next();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let input = "123";
        let mut parser = parser::new(input);
        assert_eq!(parser.next(), Some(Token::Integer(123)));
    }
}


