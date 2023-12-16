#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    Int(i64),
    Float(f64),
    Identifier(String),
    Bool(bool),
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
