//parser.rs for cube
// takes in a Vec<tokens> and returns a Ast Tree
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Token, is_keyword, ASTNode};



pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn consume(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().map(|v| &**v)
    }

    fn consume_identifier(&mut self) -> String {
        match self.consume() {
            Some(Token::Identifier(ident)) => ident.clone(),
            _ => panic!("Expected Identifier from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn consume_number(&mut self) -> f64 {
        match self.consume() {
            Some(Token::Number(num)) => *num,
            _ => panic!("Expected Number from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn consume_string(&mut self) -> String {
        match self.consume() {
            Some(Token::String(s)) => s.clone(),
            _ => panic!("Expected String from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn consume_boolean(&mut self) -> bool {
        match self.consume() {
            Some(Token::Boolean(b)) => *b,
            _ => panic!("Expected Boolean from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn consume_keyword(&mut self) -> String {
        match self.consume() {
            Some(Token::Keyword(kw)) => kw.clone(),
            _ => panic!("Expected Keyword from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn consume_operator(&mut self) -> String {
        match self.consume() {
            Some(Token::Operator(op)) => op.clone(),
            _ => panic!("Expected Operator from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while let Some(Token::Operator(op)) = self.peek() {
            if op == "+" || op == "-" {
            
                node = ASTNode::Function(op.clone(), vec![node, self.parse_term()]);
            } else {
                break;
            }
        }
        node
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while let Some(Token::Operator(op)) = self.peek() {
            if op == "*" || op == "/" {
              
                node = ASTNode::Function(op.clone(), vec![node, self.parse_factor()]);
            } else {
                break;
            }
        }
        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        match &self.peek() {
            Some(Token::Number(_)) => ASTNode::Number(self.consume_number()),
            Some(Token::Identifier(_)) => ASTNode::Identifier(self.consume_identifier()),
            Some(Token::String(_)) => ASTNode::String(self.consume_string()),
            Some(Token::Boolean(_)) => ASTNode::Boolean(self.consume_boolean()),
            Some(Token::Operator(op)) if op == "+" || op == "-" => {
              
                ASTNode::Function(op.clone(), vec![self.parse_factor()])
            }
            Some(Token::Operator(op)) if op == "(" => {
                self.consume();
                let node = &self.parse_expression();
                if let Some(Token::Operator(op)) = &self.peek() {
                    if op == ")" {
                        self.consume();
                        return node.clone();
                    }
                }
                panic!("Expected )");
            }
            _ => panic!("Expected Number, Identifier, String, Boolean, or ( from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn parse_statement(&mut self) -> ASTNode {
        println!("{:?}", self.peek());
        match self.peek() {
            Some(Token::Keyword(kw)) => {
                match kw.as_str() {
                    "if" => self.parse_if(),
                    "while" => self.parse_while(),
                    "for" => self.parse_for(),
                    "return" => self.parse_return(),
                    "}" => panic!("Invalid Keyword"),
                    "print" => {
                        self.consume_keyword();
                        let node = self.parse_expression();
                        ASTNode::Function("print".to_string(), vec![node])
                    }
                    _ => panic!("Invalid Keyword"),
                }
            }
            Some(Token::Identifier(_)) => self.parse_assignment(),
            Some(Token::Operator(op)) if op == "{" => {
                self.consume();
                let block = self.parse_block();
                ASTNode::Function("block".to_string(), block)
            }
            // function call
            Some(Token::Operator(op)) if op == "(" => {
                let node = self.parse_expression();
                ASTNode::Function("function_call".to_string(), vec![node])
            }

            Some(Token::Operator(op)) if op == ";" => {
                self.consume();
                ASTNode::SemiColon
            }

            _ => panic!("Expected Keyword or Identifier from {} at {}", self.peek().cloned().unwrap(), &self.tokens.len()),
        }
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut block = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Keyword(kw) => {
                    match kw.as_str() {
                        "}" => {
                            self.consume();
                            break;
                        }
                        _ => block.push(self.parse_statement()),
                    }
                }
                _ => block.push(self.parse_statement()),
            }
        }
        block
    }

    fn parse_if(&mut self) -> ASTNode {
        self.consume_keyword();
        let condition = self.parse_expression();
        let block = self.parse_block();
        let mut else_block = Vec::new();
        if let Some(Token::Keyword(kw)) = self.peek() {
            if kw == "else" {
                self.consume_keyword();
                else_block = self.parse_block();
            }
        }
        ASTNode::Function("if".to_string(), vec![condition, ASTNode::Function("block".to_string(), block), ASTNode::Function("block".to_string(), else_block)])
    }

    fn parse_while(&mut self) -> ASTNode {
        self.consume_keyword();
        let condition = self.parse_expression();
        let block = self.parse_block();
        ASTNode::Function("while".to_string(), vec![condition, ASTNode::Function("block".to_string(), block)])
    }

    fn parse_for(&mut self) -> ASTNode {
        self.consume_keyword();
        let init = self.parse_statement();
        let condition = self.parse_expression();
        let update = self.parse_statement();
        let block = self.parse_block();
        ASTNode::Function("for".to_string(), vec![init, condition, update, ASTNode::Function("block".to_string(), block)])
    }

    fn parse_return(&mut self) -> ASTNode {
        self.consume_keyword();
        let node = self.parse_expression();
        ASTNode::Function("return".to_string(), vec![node])
    }

    fn parse_assignment(&mut self) -> ASTNode {
        let ident = self.consume_identifier();
        self.consume_operator();
        let node = self.parse_expression();
        ASTNode::Function("=".to_string(), vec![ASTNode::Identifier(ident), node])
    }
    fn parse_identifier(&mut self) -> ASTNode {
        // identifier could be a function call or a variable
        let ident = self.consume_identifier();
        if let Some(Token::Operator(op)) = self.peek() {
            if op == "(" {
                self.consume();
                let mut args = Vec::new();
                while let Some(Token::Identifier(ident)) = self.peek() {
                    args.push(ASTNode::Identifier(ident.clone()));
                    self.consume();
                    if let Some(Token::Operator(op)) = self.peek() {
                        if op == ")" {
                            self.consume();
                            break;
                        }
                    }
                }
                ASTNode::Function(ident, args)
            } else {
                ASTNode::Identifier(ident)
            }
        } else {
            ASTNode::Identifier(ident)
        }

    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut ast = Vec::new();
        while let Some(_) = self.peek() {
            ast.push(self.parse_statement());
        }
        ast
    }
}

impl<'a> fmt::Display for Parser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser")
    }
}

pub fn parser(tokens: &Vec<Token>) -> Vec<ASTNode> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}



