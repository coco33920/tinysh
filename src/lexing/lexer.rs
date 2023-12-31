use std::str::Chars;

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
        || character == '.'
        || character == '"'
        || character == ' ';
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

    fn lex_float(self, whole_side: i64, chars: &mut Chars) -> f64 {
        let current_char_options = chars.next();
        let current_char = match current_char_options {
            Some(t) => t,
            None => '0',
        };
        let a = self.lex_raddix(chars, Some(current_char));
        let f = (&*(whole_side.to_string().as_str().to_owned() + "." + a.as_str())).parse();
        if f.is_err() {
            return f64::NAN;
        }
        f.unwrap()
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
        let mut quote = 0;
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
                '"' => {
                    quote += 1;
                    Token::Quote
                }
                ' ' => {
                    if quote % 2 == 1 {
                        Token::Whitespace
                    } else {
                        Token::Null
                    }
                }
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
                        let mut next = char_iter.clone().peekable();
                        match next.peek() {
                            Some(p) if *p == '.' => {
                                char_iter.next();
                                let f = self.clone().lex_float(a, &mut char_iter);
                                Token::Float(f)
                            }
                            _ => Token::Int(a),
                        }
                    } else if ch.is_alphabetic() {
                        let str = self.clone().lex_string(&mut char_iter, Some(ch));
                        match str.as_str() {
                            "false" => Token::Bool(false),
                            "true" => Token::Bool(true),
                            "or" => Token::Or,
                            "and" => Token::And,
                            _ => Token::Identifier(str),
                        }
                    } else if ch == '.' {
                        let f = self.clone().lex_float(0, &mut char_iter);
                        Token::Float(f)
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
        let mut final_vec = Vec::new();
        vec.into_iter()
            .filter(|x| &Token::PreAnd != x)
            .for_each(|f| final_vec.push(f));
        final_vec
    }
}

#[cfg(test)]
mod test {

    use crate::lexing::token::Token;

    use super::{is_an_allowed_character, Lexer};

    #[test]
    pub fn test_allowed() {
        let expected = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '(', ')', '.', ' ', '"', ' ',
        ];
        let value = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '^', '(', ')', '%', '.', ' ',
            '$', '"', ' ',
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
            Token::Quote,
            Token::Whitespace,
            Token::Quote,
        ];
        let value = Lexer {
            str: "|)(><-~&&\" \" ".to_string(),
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

    #[test]
    pub fn remove_preand() {
        let expected = vec![];
        let value = Lexer {
            str: "&".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }

    #[test]
    pub fn simple_float() {
        let expected = vec![Token::Float(0.1)];
        let value = Lexer {
            str: ".1".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }

    #[test]
    pub fn harder_float() {
        let expected = vec![Token::Float(1.1)];
        let value = Lexer {
            str: "1.1".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }

    #[test]
    pub fn float_in_expr() {
        let expected = vec![Token::Float(1.1), Token::And, Token::Float(2.2)];
        let value = Lexer {
            str: "1.1 && 2.2".to_string(),
        };
        assert_eq!(value.lex(), expected);
    }

    #[test]
    pub fn test_lex_string_error() {
        let expected = 0;
        let mut chars = "ss".chars();
        let first_char = chars.next().unwrap();
        let value = Lexer {
            str: "str".to_string(),
        }
        .lex_int(&mut chars, first_char);
        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_lex_float_error_nan() {
        let mut chars = "ss".chars();
        let _first_char = chars.next().unwrap();
        let value = Lexer {
            str: "str".to_string(),
        }
        .lex_float(0, &mut chars);
        assert_eq!(true, value.is_nan());
    }

    #[test]
    pub fn test_lex_float_error() {
        let expected = 0f64;
        let mut chars = "s".chars();
        let _ = chars.next();
        let value = Lexer {
            str: "str".to_string(),
        }
        .lex_float(0, &mut chars);
        assert_eq!(value, expected);
    }
}
