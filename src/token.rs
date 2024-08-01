
use std::{iter::Peekable, str::Chars};

use crate::lexer::Lexer;


#[derive(Debug, PartialEq)]
pub enum Token {
    // Data types
    Int,
    Float,
    Str,
    Bool,
    Void,
    // Keywords
    If,
    Else,
    While,
    For,
    Print,
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    Empty,
    Not,
    // Symbols
    Colon,
    Semicolon,
    NewLine,

    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // Literals
    Identifier(String),
    Integer(i32),
    CFloat(f64),
    String(String),
    // End of file
    Eof,
}
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_identifier() {
        let input = "hello";
        let mut tokenizer = Tokenizer::new(input);
        let ident = tokenizer.read_identifier('h');
        assert_eq!(ident, "hello");
    }
}
