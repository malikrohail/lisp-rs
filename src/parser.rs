use crate::lexer::*;
use crate::object::*;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError {
    token: Token,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected token: {}", self.token)
    }
}

impl Error for ParseError {}
    


pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {

    let token = tokens.pop();
    if token != Some(Token::LParen) {
        println!("Did not find LParen, found {:?}, remaining tokens {:?}", token, tokens);
        return Err(ParseError {
            token: token.unwrap(),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            println!("Did not find enough tokens");
            return Err(ParseError {
                token: Token::Invalid,
            });
        }
        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Float(n) => list.push(Object::Float(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list( tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
            _ => {
                println!("Unexpected token {:?}", t);
                return Err(ParseError {
                    token: Token::Invalid,
                })
            }
        }
    }

    Ok(Object::List(list))
}
