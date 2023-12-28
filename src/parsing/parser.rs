use std::slice::Iter;

use crate::lexing::token::{Token, TokenType};

use super::{
    ast::Ast,
    parselets::{
        infix_parselet::{InfixParselet, NullParset, OperatorInfixParselet},
        prefix_parselet::{
            GroupParselet, NullParselet, OperatorPrefixParselet, PrefixParselet, QuoteParselet,
            ValueParselet,
        },
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

    #[cfg(not(tarpaulin_include))]
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
            TokenType::Or => Some(Box::from(OperatorInfixParselet {
                is_right: false,
                precedence: 10,
            })),
            TokenType::And => Some(Box::from(OperatorInfixParselet {
                is_right: false,
                precedence: 10,
            })),
            TokenType::Pipe => Some(Box::from(OperatorInfixParselet {
                is_right: false,
                precedence: 8,
            })),
            TokenType::LeftRedirection => Some(Box::from(OperatorInfixParselet {
                is_right: false,
                precedence: 9,
            })),
            TokenType::RightRedirection => Some(Box::from(OperatorInfixParselet {
                is_right: false,
                precedence: 9,
            })),
            _ => Some(Box::from(NullParset {})),
        }
    }

    pub fn get_prefix_parselet(self, token_type: TokenType) -> Option<Box<dyn PrefixParselet>> {
        match token_type {
            TokenType::Int => Some(Box::from(ValueParselet {})),
            TokenType::Float => Some(Box::from(ValueParselet {})),
            TokenType::Identifier => Some(Box::from(ValueParselet {})),
            TokenType::Bool => Some(Box::from(ValueParselet {})),
            TokenType::Or => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::And => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::LPar => Some(Box::from(GroupParselet {})),
            TokenType::LeftRedirection => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::RightRedirection => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::Pipe => Some(Box::from(OperatorPrefixParselet {})),
            TokenType::Quote => Some(Box::from(QuoteParselet {})),
            _ => Some(Box::from(NullParselet {})),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexing::{lexer::Lexer, token::Token},
        parsing::ast::{Ast, Parameters},
    };

    use super::init_calc_parser;

    #[test]
    pub fn test_parser() {
        let data = Lexer {
            str: "(".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        assert_eq!(
            parser.consume_expected(crate::lexing::token::TokenType::Or),
            Token::Null
        );
        assert_eq!(parser.consume(), Token::Null);
        assert_eq!(
            parser.consume_expected(crate::lexing::token::TokenType::And),
            Token::Null
        );
    }

    #[test]
    pub fn test_parse_int() {
        let expected = Ast::Node {
            value: Parameters::Int(1),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "1".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected)
    }

    #[test]
    pub fn test_parse_float() {
        let expected = Ast::Node {
            value: Parameters::Float(1.0),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "1.0".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected)
    }

    #[test]
    pub fn test_parse_false() {
        let expected = Ast::Node {
            value: Parameters::Bool(false),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "false".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected)
    }

    #[test]
    pub fn test_parse_identifier() {
        let expected = Ast::Node {
            value: Parameters::Identifier("test".to_string()),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "test".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected)
    }

    #[test]
    pub fn test_parse_string() {
        let expected = Ast::new(Parameters::Str("test 1 2 1 2".to_string()));
        let data = Lexer {
            str: "\"test 1 2 1 2\"".to_string(),
        };
        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected)
    }

    #[test]
    pub fn test_parse_and_prefix() {
        let expected = Ast::Node {
            value: Parameters::And,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "&& true".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_parse_or_prefix() {
        let expected = Ast::Node {
            value: Parameters::Or,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "or true".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_parse_leftredirect_prefix() {
        let expected = Ast::Node {
            value: Parameters::LeftRedirection,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "> true".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_parse_rightredirect_prefix() {
        let expected = Ast::Node {
            value: Parameters::RightRedirection,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "< true".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_parse_pipe_prefix() {
        let expected = Ast::Node {
            value: Parameters::Pipe,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "| true".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_group_prefix() {
        let expected = Ast::Node {
            value: Parameters::Pipe,
            left: Box::from(Ast::new(Parameters::Bool(true))),
            right: Box::from(Ast::Nil),
        };

        let data = Lexer {
            str: "(| true)".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_null_parselet() {
        let expected = Ast::Nil;
        let data = Lexer {
            str: "&".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_and_infix() {
        let expected = Ast::Node {
            value: Parameters::And,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let data = Lexer {
            str: "1 && 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_or_infix() {
        let expected = Ast::Node {
            value: Parameters::Or,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let data = Lexer {
            str: "1 or 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_pipe_infix() {
        let expected = Ast::Node {
            value: Parameters::Pipe,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let data = Lexer {
            str: "1 | 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_leftredirection_infix() {
        let expected = Ast::Node {
            value: Parameters::LeftRedirection,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let data = Lexer {
            str: "1 > 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_rightredirection_infix() {
        let expected = Ast::Node {
            value: Parameters::RightRedirection,
            left: Box::from(Ast::new(Parameters::Int(1))),
            right: Box::from(Ast::new(Parameters::Int(1))),
        };
        let data = Lexer {
            str: "1 < 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_null_infix() {
        let expected = Ast::Node {
            value: Parameters::Int(1),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let data = Lexer {
            str: "1 & 1".to_string(),
        };

        let datalex = data.lex();
        let parser = &mut init_calc_parser(&datalex);
        let value = parser.parse();
        assert_eq!(value, expected);
    }
}
