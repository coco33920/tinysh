use crate::{
    lexing::token::Token,
    parsing::{ast::Ast, parser::Parser},
};

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, left: &Ast, token: Token) -> Ast;
    fn get_precedence(&self) -> i64;
}

pub struct NullParset {}

impl InfixParselet for NullParset {
    fn parse(&self, _parser: &mut Parser, left: &Ast, _token: Token) -> Ast {
        left.clone()
    }

    fn get_precedence(&self) -> i64 {
        0
    }
}
