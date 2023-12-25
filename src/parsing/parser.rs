use std::slice::Iter;

use crate::lexing::token::{Token, TokenType};

use super::ast::Ast;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    read: Vec<Token>,
}

pub fn init_calc_parser(input: &Vec<Token>) -> Parser {
    Parser {
        tokens: input.iter(),
        read: Vec::new(),
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Ast {
        Ast::Nil
    }

    fn look_ahead(&mut self, distance: usize) -> Token {
        while distance >= self.read.len() {
            match self.tokens.next() {
                None => break,
                Some(t) => self.read.push(t.clone()),
            }
        }
        match self.read.get(distance) {
            None => Token::Null,
            Some(t) => t.clone(),
        }
    }
    pub fn consume(&mut self) -> Token {
        self.look_ahead(0);
        if self.read.len() == 0 {
            return Token::Null;
        }
        self.read.remove(0)
    }

    pub fn match_token(&mut self, expected: TokenType) -> bool {
        let token = self.look_ahead(0);
        if token.to_token_type() != expected {
            return false;
        }
        return true;
    }

    pub fn consume_expected(&mut self, expected: TokenType) -> Token {
        self.look_ahead(0);
        if self.read.len() == 0 {
            return Token::Null;
        }
        match self.read.remove(0) {
            t => {
                if t.to_token_type() == expected {
                    t
                } else {
                    println!("error!");
                    Token::Null
                }
            }
        }
    }
}
