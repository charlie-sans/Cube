// token.rs

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Function(String, Vec<Token>),
    Operator(String),
    Keyword(String),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f64),
    Void,
    SemiColon,
    
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "Identifier: {}", ident),
            Token::Number(num) => write!(f, "Number: {}", num),
            Token::Function(ident, tokens) => {
                write!(f, "Function: {}(", ident)?;
                for token in tokens {
                    write!(f, "{}, ", token)?;
                }
                write!(f, ")")
            }
            Token::Operator(op) => write!(f, "Operator: {}", op),
            Token::Keyword(kw) => write!(f, "Keyword: {}", kw),
            Token::String(s) => write!(f, "String: {}", s),
            Token::Boolean(b) => write!(f, "Boolean: {}", b),
            Token::EOF => write!(f, "EOF"),
            Token::Int(i) => write!(f, "Int: {}", i),
            Token::Float(fl) => write!(f, "Float: {}", fl),
            Token::Void => write!(f, "Void"),
            Token::SemiColon => write!(f, "SemiColon"),
        }
    }
}

pub fn is_keyword(ident: &str) -> bool {
    match ident {
        "if" | "else" | "for" | "while" | "return" | "void" | "int" | "float" | "str" | "bool" => true,
        _ => false,
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Function(String, Vec<ASTNode>),
    Number(f64),
    Identifier(String),
    Operator(String),
    Keyword(String),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f64),
    Void,
    SemiColon,
    EOF,
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Identifier(ident) => write!(f, "Identifier: {}", ident),
            ASTNode::Number(num) => write!(f, "Number: {}", num),
            ASTNode::Function(ident, tokens) => {
                write!(f, "Function: {}(", ident)?;
                for token in tokens {
                    write!(f, "{}, ", token)?;
                }
                write!(f, ")")
            }
            ASTNode::Operator(op) => write!(f, "Operator: {}", op),
            ASTNode::Keyword(kw) => write!(f, "Keyword: {}", kw),
            ASTNode::String(s) => write!(f, "String: {}", s),
            ASTNode::Boolean(b) => write!(f, "Boolean: {}", b),
            ASTNode::Int(i) => write!(f, "Int: {}", i),
            ASTNode::Float(fl) => write!(f, "Float: {}", fl),
            ASTNode::Void => write!(f, "Void"),
            ASTNode::SemiColon => write!(f, "SemiColon"),
            ASTNode::EOF => write!(f, "EOF"),
        }
    }
}

