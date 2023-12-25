use crate::{
    lexing::token::Token,
    parsing::{ast::Ast, parser::Parser},
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Ast;
}

pub struct NullParselet {}

impl PrefixParselet for NullParselet {
    fn parse(&self, _parser: &mut Parser, _token: Token) -> Ast {
        Ast::Nil
    }
}
