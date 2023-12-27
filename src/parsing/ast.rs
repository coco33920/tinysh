use std::fmt::Display;

use crate::lexing::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Parameters {
    Int(i64),
    Float(f64),
    Identifier(String),
    Str(String),
    Bool(bool),
    Pipe,
    LeftRedirection,
    RightRedirection,
    Assign,
    And,
    Or,
    Null,
    Call(String, Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Nil,
    Node {
        value: Parameters,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Parameters::Int(i) => write!(f, "{i}"),
            Parameters::Float(fs) => write!(f, "{fs}"),
            Parameters::Identifier(s) => write!(f, "{s}"),
            Parameters::Str(s) => write!(f, "{s}"),
            Parameters::Bool(b) => write!(f, "{b}"),
            Parameters::Pipe => write!(f, "|"),
            Parameters::LeftRedirection => write!(f, ">"),
            Parameters::RightRedirection => write!(f, "<"),
            Parameters::Assign => write!(f, "="),
            Parameters::And => write!(f, "&&"),
            Parameters::Or => write!(f, "or"),
            Parameters::Null => write!(f, ""),
            Parameters::Call(s, l) => write!(f, "{} {}", s, l.join(" ")),
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Nil => write!(f, ""),
            Ast::Node { value, left, right } => {
                write!(f, "{}", format!("{} {} {}", left, value, right).trim())
            }
        }
    }
}

pub fn token_to_parameter(token: Token) -> Parameters {
    match token {
        Token::Or => Parameters::Or,
        Token::And => Parameters::And,
        Token::Pipe => Parameters::Pipe,
        Token::Int(s) => Parameters::Int(s.clone()),
        Token::Float(s) => Parameters::Float(s.clone()),
        Token::Identifier(s) => Parameters::Identifier(s.clone()),
        Token::Bool(b) => Parameters::Bool(b.clone()),
        Token::LeftRedirection => Parameters::LeftRedirection,
        Token::RightRedirection => Parameters::RightRedirection,
        _ => Parameters::Null,
    }
}

impl Ast {
    pub fn new(p: Parameters) -> Ast {
        Ast::Node {
            value: p,
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexing::token::Token;

    use super::{token_to_parameter, Ast, Parameters};

    #[test]
    fn test_display_parameters() {
        let expected = "[5, 5.5, test, test2, false, |, >, <, =, &&, or, ls color, ]";
        let value = vec![
            Parameters::Int(5),
            Parameters::Float(5.5),
            Parameters::Identifier("test".to_string()),
            Parameters::Str("test2".to_string()),
            Parameters::Bool(false),
            Parameters::Pipe,
            Parameters::LeftRedirection,
            Parameters::RightRedirection,
            Parameters::Assign,
            Parameters::And,
            Parameters::Or,
            Parameters::Call("ls".to_string(), vec!["color".to_string()]),
            Parameters::Null,
        ];
        let mut final_vec = Vec::new();
        value
            .into_iter()
            .map(|f| f.to_string())
            .for_each(|f| final_vec.push(f));
        let value_e = format!("[{}]", final_vec.join(", "));
        assert_eq!(format!("{value_e}"), expected);
    }

    #[test]
    fn test_display_nil_ast() {
        let expected = "";
        let value = format!("{}", Ast::Nil);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_display_ast_leaf() {
        let expected = "5";
        let v = Ast::new(Parameters::Int(5));
        let value = format!("{v}");
        assert_eq!(value, expected);
    }

    #[test]
    fn test_token_to_parameter() {
        let expected = vec![
            Parameters::Identifier("t".to_string()),
            Parameters::Int(5),
            Parameters::Float(5.5),
            Parameters::Or,
            Parameters::And,
            Parameters::Pipe,
            Parameters::LeftRedirection,
            Parameters::RightRedirection,
            Parameters::Bool(false),
            Parameters::Null,
        ];
        let v = vec![
            Token::Identifier("t".to_string()),
            Token::Int(5),
            Token::Float(5.5),
            Token::Or,
            Token::And,
            Token::Pipe,
            Token::LeftRedirection,
            Token::RightRedirection,
            Token::Bool(false),
            Token::PreAnd,
        ];
        let mut value = Vec::new();
        v.into_iter()
            .map(|f| token_to_parameter(f))
            .for_each(|v| value.push(v));
        assert_eq!(value, expected);
    }
}
