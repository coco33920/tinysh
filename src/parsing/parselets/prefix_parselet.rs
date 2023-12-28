use crate::{
    lexing::token::{Token, TokenType},
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

pub struct OperatorPrefixParselet {}

pub struct GroupParselet {}

pub struct QuoteParselet {}

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

impl PrefixParselet for OperatorPrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Ast {
        let operand = parser.parse_expression_empty();
        Ast::Node {
            value: token_to_parameter(token),
            left: Box::from(operand),
            right: Box::from(Ast::Nil),
        }
    }
}

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> Ast {
        let expression = parser.parse_expression_empty();
        parser.consume_expected(crate::lexing::token::TokenType::RPar);
        expression
    }
}

impl PrefixParselet for QuoteParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> Ast {
        let mut str: String = String::new();

        if !parser.match_token(TokenType::Quote) {
            while !parser.match_token(TokenType::Quote) {
                match parser.consume() {
                    Token::Identifier(s) => str = str + &s.to_string(),

                    t => str = str + &t.to_string(),
                }
            }
            parser.consume_expected(TokenType::Quote);
        }

        Ast::Node {
            value: crate::parsing::ast::Parameters::Str(str.trim().to_string()),
            left: Box::new(Ast::Nil),
            right: Box::new(Ast::Nil),
        }
    }
}
