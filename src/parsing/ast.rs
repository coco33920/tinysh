use std::fmt::Display;

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
    Call(String, Vec<String>),
}

#[derive(Debug, Clone)]
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
    use super::{Ast, Parameters};

    #[test]
    fn test_display_parameters() {
        let expected = "[5, 5.5, test, test2, false, |, >, <, =, &&, or, ls color]";
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
}
