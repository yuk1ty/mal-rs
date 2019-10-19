#[derive(Debug, PartialEq)]
pub enum MalType {
    Nil,
    Int(String),
    String(String),
    Bool(bool),
    Symbol(String),
    List(Box<Vec<MalType>>, Box<MalType>),
    Vector(Box<Vec<MalType>>, Box<MalType>),
    HashMap(Box<Vec<MalType>>, Box<MalType>),
}
