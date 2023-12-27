use crate::{
    lexing::token::Token,
    parsing::{
        ast::{token_to_parameter, Ast},
        parser::Parser,
    },
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Ast;
}

pub struct NullParselet {}

pub struct ValueParselet {}

impl PrefixParselet for ValueParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Ast {
        Ast::Node {
            value: token_to_parameter(token),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for NullParselet {
    fn parse(&self, _parser: &mut Parser, _token: Token) -> Ast {
        Ast::Nil
    }
}
