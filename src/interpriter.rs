

use crate::Token;

fn print_f(msg: &str) {
    println!("{}", msg);
}
//𓎢𓄿𓈖 𓇌𓅲 𓅲𓈖𓂧𓂋𓋴𓏏𓄿𓈖𓂧 𓅓?
pub fn interpret(tokens: Vec<Token>) {
   let mut i = 0;
   let mut varibles = Vec::new();
   let mut objects: Vec<(String, String)> = Vec::new();
   let mut functions: Vec<(String, Vec<Token>)> = Vec::new();
   let mut classes: Vec<(String, Vec<Token>)> = Vec::new();
   let mut function_args: Vec<(String, Vec<Token>)> = Vec::new();
 

    while i < tokens.len() {
         match tokens[i] {
                Token::NewLine => {
                    i += 1;
                },
                Token::Identifier(ref ident) => {
                    i += 1;
                    match tokens[i] {
                        Token::Colon => {
                            i += 1;
                            match tokens[i] {
                                // check if the token is a Int,String,Float, or Bool
                                Token::Int | Token::Str | Token::Float | Token::Bool | Token::Void => {
                                    i += 1;
                                    match tokens[i] {
                                        Token::Equals => {
                                            i += 1;
                                            match tokens[i] {
                                                Token::Integer(ref int) => {
                                                    // if the token is a int folowed by a semicolon
                                                    if tokens[i + 1] == Token::Semicolon {
                                                        varibles.push((ident.clone(), int.to_string()));
                                                        break;
                                                    } else {
                                                        panic!("Expected a semicolon after the integer");
                                                    }
                                                },
                                                Token::String(ref string) => {
                                                  if tokens[i + 1] == Token::Semicolon {
                                                        varibles.push((ident.clone(), string.clone()));
                                                        break;
                                                    } else {
                                                        panic!("Expected a semicolon after the string");
                                                    }
                                                },
                                                Token::CFloat(ref float) => {
                                                    if tokens[i - 2] == Token::Float || tokens[i - 3] == Token::Semicolon {
                                                        varibles.push((ident.clone(), float.to_string()));
                                                    } else {
                                                        panic!("Expected a float after equals and a semicolon after the float");
                                                    }
                                                },
                                                
                                                _ => {
                                                    panic!("Expected an integer, string, float, or boolean after equals");
                                                }
                                            }
                                        },
                                        Token::LeftParen => {
                                            i += 1;
                                            let mut args: Vec<Token> = Vec::new();
                                            while tokens[i] != Token::RightParen {
                                                args.push(tokens[i].clone());
                                                i += 1;
                                            }
                                            function_args.push((ident.clone(), args));

                                        },
                                        Token::LeftBrace => {
                                            i += 1;
                                            let mut statements: Vec<Token> = Vec::new();
                                            while tokens[i] != Token::RightBrace {
                                                statements.push(tokens[i].clone());
                                                i += 1;
                                            }
                                            classes.push((ident.clone(), statements));
                                        },
                                        Token::Comma => {
                                            i += 1;
                                            match tokens[i] {
                                                Token::Identifier(ref ident) => {
                                                    objects.push((ident.clone(), ident.clone()));
                                                },
                                                _ => {
                                                    panic!("Expected an identifier after comma {:?} at line {}", tokens[i], i);
                                                }
                                            }
                                        }
                                                
                                            
                                        _ => {
                                            println!("Unexpected token {:?} at line {}", tokens[i], i);
                                         
                                        }
                                    }
                                },
                                
                                _ => {
                                    break;
                                }
                            }
                        },
                        Token::LeftParen => {
                            i += 1;
                            let mut args: Vec<Token> = Vec::new();
                            while tokens[i] != Token::RightParen {
                                args.push(tokens[i].clone());
                                i += 1;
                            }
                            functions.push((ident.clone(), args));
                        },
                        _ => {
                            println!("Expected a colon or left parenthesis after identifier {:?} at line {}", tokens[i], i);
                        }
                    }
                },
                Token::Print => {
                i += 1;
                match tokens[i] {
                     Token::String(ref msg) => {
                          print_f(msg);
                     },
                     Token::Identifier(ref ident) => {
                          print_f(ident);
                     },
                    
                     _ => {
                          panic!("Expected a string or identifier after print {:?} at line {}", tokens[i], i);
                     }
                     // should end with a right parenthesis
                     Token::RightParen => {
                          i += 1;
                     },
                }
                i += 1;
              },
              _ => {
                     println!("Unexpected token {:?} at line {}", tokens[i], i);
              }
         }
         i += 1;
    }
    println!("{:?}", varibles);
    println!("{:?}", objects);
    
}