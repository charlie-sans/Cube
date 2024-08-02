use crate::token::Token;
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

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
    symbol_table: HashMap<String, ASTNode>,
}


impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter().peekable(),
            symbol_table: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        self.program();
    }

    fn program(&mut self) {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Keyword(keyword) => {
                    match keyword.as_str() {
                        "int" => {
                            self.declare();
                        }
                        "void" => {
                            self.function();
                        }
                        "print" => {
                            self.parse_expression();
                        }
                        "main" => {
                            self.function();
                        }
                        _ => {
                            panic!("Unexpected keyword: {}", keyword);
                        }
                    }
                }
                _ => {
                    panic!("Unexpected token: {:?}", token);
                }
            }
        }
    }

    fn declare(&mut self) {
        let token = self.tokens.next().unwrap();
        let name = match token {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier, found {:?}", token),
        };

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != "=" {
                    panic!("Expected '=', found {}", operator);
                }
            }
            _ => panic!("Expected '=', found {:?}", token),
        }

        let token = self.tokens.next().unwrap();
        let value = match token {
            Token::Number(value) => *value as i32,
            _ => panic!("Expected number, found {:?}", token),
        };

        self.symbol_table.insert(name.clone(), ASTNode::Int(value as i32));
    }

    fn function(&mut self) {
        let token = self.tokens.next().unwrap();
        let name = match token {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier, found {:?}", token),
        };

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != "(" {
                    panic!("Expected '(', found {}", operator);
                }
            }
            _ => panic!("Expected '(', found {:?}", token),
        }

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != ")" {
                    panic!("Expected ')', found {}", operator);
                }
            }
            _ => panic!("Expected ')', found {:?}", token),
        }

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != "{" {
                    panic!("Expected '{{', found {}", operator);
                }
            }
            _ => panic!("Expected '{{', found {:?}", token),
        }

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Keyword(keyword) => {
                    match keyword.as_str() {
                        "int" => {
                            self.declare();
                        }
                        _ => {
                            panic!("Unexpected keyword: {}", keyword);
                        }
                    }
                }
                Token::Identifier(name) => {
                    self.statement();
                }
                _ => {
                    panic!("Unexpected token: {:?}", token);
                }
            }
        }

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != "}" {
                    panic!("Expected '}}', found {}", operator);
                }
            }
            _ => panic!("Expected '}}', found {:?}", token),
        }

        self.symbol_table.insert(name.clone(), ASTNode::Function(name.clone(), vec![]));
    }

    fn statement(&mut self) {
        let token = self.tokens.next().unwrap();
        let name = match token {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier, found {:?}", token),
        };

        let token = self.tokens.next().unwrap();
        match token {
            Token::Operator(operator) => {
                if operator != "=" {
                    panic!("Expected '=', found {}", operator);
                }
            }
            _ => panic!("Expected '=', found {:?}", token),
        }

        let token = self.tokens.next().unwrap();
        let value = match token {
            Token::Identifier(name) => {
                match self.symbol_table.get(name) {
                    Some(ASTNode::Int(value)) => *value,
                    _ => panic!("Expected int, found {:?}", token),
                }
            }
            Token::Number(value) => *value as i32,
            _ => panic!("Expected number, found {:?}", token),
        };

        self.symbol_table.insert(name.clone(), ASTNode::Int(value));
    }
    fn parse_expression(&mut self) {
        let token = self.tokens.next().unwrap();
        match token {
            Token::Identifier(name) => {
                match self.symbol_table.get(name) {
                    Some(ASTNode::Int(value)) => {
                        self.parse_expression();
                    }
                    _ => panic!("Expected int, found {:?}", token),
                }
            }
            Token::Number(value) => {
                self.parse_expression();
            }
            _ => panic!("Expected number, found {:?}", token),
        }
    }
}

pub fn interpret(tokens: Vec<Token>) {
    let mut parser = Parser::new(&tokens);
    parser.parse();
}
