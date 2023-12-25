use crate::{
    lexing::token::Token,
    parsing::{ast::Ast, parser::Parser},
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Ast;
}
