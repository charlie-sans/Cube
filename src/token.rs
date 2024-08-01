
use std::{iter::Peekable, str::Chars};
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Statement(Vec<ASTNode>),
    Expression(String, Vec<ASTNode>),
    Function(String, Vec<Token>,Vec<ASTNode>),
    Varible(String, Vec<ASTNode>),
    Class(String, Vec<ASTNode>),

}

#[derive(Debug, PartialEq, Clone)]
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
    Function(String, Vec<Token>,Vec<Token>),
    Varible(String, Vec<Token>),
    class(String, Vec<Token>),
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding: &str = "binding";
        let token_str = match self {
            Token::Int => "Int",
            Token::Float => "Float",
            Token::Str => "Str",
            Token::Bool => "Bool",
            Token::Void => "Void",
            Token::If => "If",
            Token::Else => "Else",
            Token::While => "While",
            Token::For => "For",
            Token::Print => "Print",
            Token::Plus => "Plus",
            Token::Minus => "Minus",
            Token::Multiply => "Multiply",
            Token::Divide => "Divide",
            Token::Modulo => "Modulo",
            Token::Equals => "Equals",
            Token::NotEquals => "NotEquals",
            Token::LessThan => "LessThan",
            Token::GreaterThan => "GreaterThan",
            Token::LessThanOrEqual => "LessThanOrEqual",
            Token::GreaterThanOrEqual => "GreaterThanOrEqual",
            Token::And => "And",
            Token::Or => "Or",
            Token::Empty => "Empty",
            Token::Not => "Not",
            Token::Colon => "Colon",
            Token::Semicolon => "Semicolon",
            Token::NewLine => "NewLine",
            Token::Comma => "Comma",
            Token::LeftParen => "LeftParen",
            Token::RightParen => "RightParen",
            Token::LeftBrace => "LeftBrace",
            Token::RightBrace => "RightBrace",
            Token::Identifier(ident) => ident,
            Token::Integer(i) => &binding,
            Token::CFloat(f) => &binding,
            _ => "Unknown",            
            Token::String(s) => s,
            Token::Eof => "Eof",
        };

        write!(f, "{}", token_str)
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
