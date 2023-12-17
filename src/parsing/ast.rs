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

pub enum Ast {
    Nil,
    Node {
        value: Parameters,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}
