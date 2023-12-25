#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    Int(i64),
    Float(f64),
    Identifier(String),
    Bool(bool),
    Quote,
    Whitespace,
    And,
    Or,
    PreAnd,
    Pipe,
    LeftRedirection,
    RightRedirection,
    LPar,
    RPar,
    Tilde,
    Dash,
    Null,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TokenType {
    Int,
    Float,
    Identifier,
    Bool,
    Quote,
    Whitespace,
    And,
    Or,
    Pipe,
    LeftRedirection,
    RightRedirection,
    LPar,
    RPar,
    Tilde,
    Dash,
    Null,
}
impl Token {
    // add code here
    pub fn to_token_type(&self) -> TokenType {
        match self {
            Token::Int(_) => TokenType::Int,
            Token::Float(_) => TokenType::Float,
            Token::Identifier(_) => TokenType::Identifier,
            Token::Bool(_) => TokenType::Bool,
            Token::Quote => TokenType::Quote,
            Token::Whitespace => TokenType::Whitespace,
            Token::And => TokenType::And,
            Token::Or => TokenType::Or,
            Token::Pipe => TokenType::Pipe,
            Token::LeftRedirection => TokenType::LeftRedirection,
            Token::RightRedirection => TokenType::RightRedirection,
            Token::LPar => TokenType::LPar,
            Token::RPar => TokenType::RPar,
            Token::Tilde => TokenType::Tilde,
            Token::Dash => TokenType::Dash,
            _ => TokenType::Null,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::ast::token_to_parameter;

    use super::{Token, TokenType};

    #[test]
    fn to_token_type() {
        let expected = vec![
            TokenType::Int,
            TokenType::Float,
            TokenType::Identifier,
            TokenType::Bool,
            TokenType::Quote,
            TokenType::Whitespace,
            TokenType::And,
            TokenType::Or,
            TokenType::Pipe,
            TokenType::LeftRedirection,
            TokenType::RightRedirection,
            TokenType::LPar,
            TokenType::RPar,
            TokenType::Tilde,
            TokenType::Dash,
            TokenType::Null,
            TokenType::Null,
        ];

        let data = vec![
            Token::Int(1),
            Token::Float(0.1),
            Token::Identifier("test".to_string()),
            Token::Bool(false),
            Token::Quote,
            Token::Whitespace,
            Token::And,
            Token::Or,
            Token::Pipe,
            Token::LeftRedirection,
            Token::RightRedirection,
            Token::LPar,
            Token::RPar,
            Token::Tilde,
            Token::Dash,
            Token::Null,
            Token::PreAnd,
        ];

        data.into_iter()
            .zip(expected.into_iter())
            .for_each(|(x, y)| assert_eq!(x.to_token_type(), y));
    }
}
