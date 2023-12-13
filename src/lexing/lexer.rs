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
        || character == ')';
}

pub struct Lexer {
    pub str: String,
}

impl Lexer {
    pub fn lex(self) -> Vec<Token> {
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
                _ => Token::Null,
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
    use super::is_an_allowed_character;

    #[test]
    pub fn test_allowed() {
        let expected = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '(', ')',
        ];
        let value = vec![
            'c', 'l', 'm', '&', '|', '>', '<', '-', '_', '0', '~', '^', '(', ')', '%',
        ];
        let mut final_value = Vec::new();
        value
            .into_iter()
            .filter(|x| is_an_allowed_character(x.clone()))
            .for_each(|f| final_value.push(f));
        assert_eq!(final_value, expected)
    }
}
