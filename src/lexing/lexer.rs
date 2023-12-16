use std::{str::Chars, thread::current};

use super::token::Token;

pub fn is_an_allowed_character(character: char) -> bool {
    return character.is_alphanumeric()
        || character == '&'
        || character == '>'
        || character == '|'
        || character == '-'
        || character == '<'
        || character == '_'
        || character == '~'
        || character == '('
        || character == ')'
        || character == '.';
}

#[derive(Clone, PartialEq)]
pub struct Lexer {
    pub str: String,
}

impl Lexer {
    fn lex_int(&self, chars: &mut Chars, current_char: char) -> i64 {
        let a = self.clone().lex_raddix(chars, Some(current_char));
        let err: Result<i64, std::num::ParseIntError> = a.parse();
        match err {
            Err(e) => {
                println!("{e}");
                0
            }
            Ok(t) => t,
        }
    }

    fn lex_raddix(self, chars: &mut Chars, mut current_char: Option<char>) -> String {
        let mut str: String = String::new();
        str += &*current_char.unwrap().to_string();
        let mut peekable = chars.clone().peekable();
        while current_char != None {
            current_char = peekable.next();
            if current_char == None {
                break;
            }
            if !current_char.unwrap().is_ascii_digit() {
                break;
            }
            current_char = chars.next();
            str += &*current_char.unwrap().to_string();
        }
        str
    }

    fn lex_string(self, chars: &mut Chars, mut current_char: Option<char>) -> String {
        let mut str: String = String::new();
        str += &*current_char.unwrap().to_string();
        let mut peekable = chars.clone().peekable();
        while current_char != None {
            current_char = peekable.next();
            if current_char == None {
                break;
            }
            if !(current_char.unwrap().is_alphanumeric() || current_char.unwrap() == '_') {
                break;
            }
            current_char = chars.next();
            str += &*current_char.unwrap().to_string();
        }

        str
    }
    pub fn lex(&self) -> Vec<Token> {
        let mut char_iter = self.str.chars();
        let mut vec = Vec::new();
        let mut char = char_iter.next();
        while char != None {
            let v = match char.unwrap() {
                p if !is_an_allowed_character(p) => Token::Null,
                '>' => Token::LeftRedirection,
                '<' => Token::RightRedirection,
                '|' => Token::Pipe,
                '~' => Token::Tilde,
                ')' => Token::RPar,
                '(' => Token::LPar,
                '-' => Token::Dash,
                '&' => {
                    let v = vec.pop();
                    match v {
                        None => Token::PreAnd,
                        Some(Token::PreAnd) => Token::And,
                        Some(p) => {
                            vec.push(p);
                            Token::PreAnd
                        }
                    }
                }
                ch => {
                    if ch.is_numeric() {
                        let a = self.lex_int(&mut char_iter, ch);
                        Token::Int(a)
                    } else if ch.is_alphabetic() {
                        let str = self.clone().lex_string(&mut char_iter, Some(ch));
                        match str.as_str() {
                            "false" => Token::Bool(false),
                            "true" => Token::Bool(true),
                            "or" => Token::Or,
                            "and" => Token::And,
                            _ => Token::Identifier(str),
                        }
                    } else {
                        Token::Null
                    }
                }
            };
            if v != Token::Null {
                vec.push(v)
            }
            char = char_iter.next();
        }
        vec
    }
}

#[cfg(test)]
mod test {
    use crate::lexing::token::Token;

    use super::{is_an_allowed_character, Lexer};

    #[test]
    pub fn test_allowed() {
        let expected = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '(', ')', '.',
        ];
        let value = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '^', '(', ')', '%', '.',
        ];
        let mut final_value = Vec::new();
        value
            .into_iter()
            .filter(|x| is_an_allowed_character(x.clone()))
            .for_each(|f| final_value.push(f));
        assert_eq!(final_value, expected)
    }

    #[test]
    pub fn test_lex_operators() {
        let expected = vec![
            Token::Pipe,
            Token::RPar,
            Token::LPar,
            Token::LeftRedirection,
            Token::RightRedirection,
            Token::Dash,
            Token::Tilde,
            Token::And,
        ];
        let value = Lexer {
            str: "|)(><-~&&".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }

    #[test]
    pub fn test_lex_int() {
        let expected = vec![Token::Int(10), Token::And, Token::Int(11)];
        let value = Lexer {
            str: "10&&11".to_string(),
        };
        assert_eq!(value.lex(), expected)
    }

    #[test]
    pub fn test_lex_str() {
        let expected = vec![Token::Identifier("test".to_string())];
        let value = Lexer {
            str: "test".to_string(),
        };
        assert_eq!(value.lex(), expected)
    }

    #[test]
    pub fn test_bool() {
        let expected = vec![Token::Bool(true), Token::And, Token::Bool(false)];
        let value = Lexer {
            str: "true&&false".to_string(),
        };
        assert_eq!(value.lex(), expected)
    }

    #[test]
    pub fn test_or_and() {
        let expected = vec![Token::And, Token::Or];
        let value = Lexer {
            str: "and or".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }
}
