use std::slice::Iter;

use crate::lexing::token::{Token, TokenType};

use super::{
    ast::Ast,
    parselets::{
        infix_parselet::{InfixParselet, NullParset},
        prefix_parselet::{NullParselet, PrefixParselet},
    },
};

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
        self.parse_expression_empty()
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

    pub fn parse_expression(&mut self, precedence: i64) -> Ast {
        let mut token = self.consume();
        let prefix = self
            .clone()
            .get_prefix_parselet(token.clone().to_token_type());

        let mut left = prefix.unwrap().parse(self, token.clone());
        while precedence < self.get_precedence() {
            token = self.consume();
            let parser = self
                .clone()
                .get_infix_parselet(token.clone().to_token_type())
                .unwrap();
            left = parser.parse(self, &left, token);
        }
        left
    }

    pub fn parse_expression_empty(&mut self) -> Ast {
        self.parse_expression(0)
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
    fn get_precedence(&mut self) -> i64 {
        let p: Option<Box<dyn InfixParselet>> = self
            .clone()
            .get_infix_parselet(self.look_ahead(0).to_token_type());
        match p {
            None => 0,
            Some(t) => (*t).get_precedence(),
        }
    }

    pub fn get_infix_parselet(self, token_type: TokenType) -> Option<Box<dyn InfixParselet>> {
        match token_type {
            _ => Some(Box::from(NullParset {})),
        }
    }

    pub fn get_prefix_parselet(self, token_type: TokenType) -> Option<Box<dyn PrefixParselet>> {
        match token_type {
            _ => Some(Box::from(NullParselet {})),
        }
    }
}
