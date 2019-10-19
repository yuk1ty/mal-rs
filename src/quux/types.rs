#[derive(Debug, PartialEq)]
pub enum MalType {
    Nil,
    Symbol(String),
    List(Box<Vec<MalType>>, Box<MalType>),
}
