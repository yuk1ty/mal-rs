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

impl MalType {
    pub fn nil() -> MalType {
        MalType::Nil
    }

    pub fn int(s: &str) -> MalType {
        MalType::Int(s.to_string())
    }

    pub fn string(s: &str) -> MalType {
        MalType::String(s.to_string())
    }

    pub fn bool(b: bool) -> MalType {
        MalType::Bool(b)
    }

    pub fn symbol(s: &str) -> MalType {
        MalType::Symbol(s.to_string())
    }

    pub fn continuous_list(l: Vec<MalType>, r: MalType) -> MalType {
        MalType::List(Box::new(l), Box::new(r))
    }

    pub fn completed_list(l: Vec<MalType>) -> MalType {
        MalType::List(Box::new(l), Box::new(MalType::Nil))
    }

    pub fn vector(v: Vec<MalType>) -> MalType {
        MalType::Vector(Box::new(v), Box::new(MalType::Nil))
    }

    pub fn hash_map(m: Vec<MalType>) -> MalType {
        MalType::HashMap(Box::new(m), Box::new(MalType::Nil))
    }
}
