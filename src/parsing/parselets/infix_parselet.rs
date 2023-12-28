use crate::{
    lexing::token::Token,
    parsing::{
        ast::{token_to_parameter, Ast},
        parser::Parser,
    },
};

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct NullParset {}

pub struct OperatorInfixParselet {
    pub is_right: bool,
    pub precedence: i64,
}

impl InfixParselet for NullParset {
    fn parse(&self, _parser: &mut Parser, left: &Ast, _token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}

impl InfixParselet for OperatorInfixParselet {
    fn parse(&self, parser: &mut Parser, left: &Ast, token: Token) -> Ast {
        let right = parser.parse_expression(if self.is_right {
            self.get_precedence() - 1
        } else {
            self.get_precedence()
        });
        let param = token_to_parameter(token);
        Ast::Node {
            value: param,
            left: Box::from(left.clone()),
            right: Box::from(right),
        }
    }
    fn get_precedence(&self) -> i64 {
        self.precedence
    }
}
