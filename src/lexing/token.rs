use std::fmt::Display;

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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{i}"),
            Token::Float(fs) => write!(f, "{fs}"),
            Token::Identifier(s) => write!(f, "{s}"),
            Token::Bool(b) => write!(f, "{b}"),
            Token::Quote => write!(f, "\""),
            Token::Whitespace => write!(f, " "),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "or"),
            Token::Pipe => write!(f, "|"),
            Token::LeftRedirection => write!(f, ">"),
            Token::RightRedirection => write!(f, "<"),
            Token::LPar => write!(f, "("),
            Token::RPar => write!(f, ")"),
            Token::Tilde => write!(f, "~"),
            Token::Dash => write!(f, "-"),
            _ => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod test {

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

    #[test]
    fn test_display() {
        let expected = vec![
            "1".to_string(),
            "0.1".to_string(),
            "test".to_string(),
            "false".to_string(),
            "\"".to_string(),
            " ".to_string(),
            "&&".to_string(),
            "or".to_string(),
            "|".to_string(),
            ">".to_string(),
            "<".to_string(),
            "(".to_string(),
            ")".to_string(),
            "~".to_string(),
            "-".to_string(),
            "".to_string(),
            "".to_string(),
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
            .zip(expected)
            .for_each(|(x, y)| assert_eq!(x.to_string(), y));
    }
}
