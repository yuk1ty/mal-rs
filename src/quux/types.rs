use crate::quux::printer::Printer;

#[derive(Debug, PartialEq, Clone)]
pub enum MalType {
    Nil,
    Int(i32),
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
        MalType::Int(s.parse().unwrap())
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

impl Printer for MalType {
    fn pr_str(&self) -> String {
        fn strconv_vec(v: &Box<Vec<MalType>>, start: &str, end: &str) -> String {
            let s: Vec<String> = (&**v).iter().map(|e| e.pr_str()).collect();
            format!("{}{}{}", start, s.join(" "), end)
        }

        match self {
            MalType::Nil => "nil".to_string(),
            MalType::Symbol(s) => s.to_string(),
            MalType::Int(i) => i.to_string(),
            MalType::String(s) => s.to_string(),
            MalType::Bool(b) => format!("{}", b),
            MalType::List(l, _) => strconv_vec(l, "(", ")"),
            MalType::Vector(l, _) => strconv_vec(l, "[", "]"),
            MalType::HashMap(l, _) => strconv_vec(l, "{", "}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quux::printer::Printer;
    use crate::quux::types::MalType;

    #[test]
    fn pr_str_works() {
        {
            let ty = MalType::int("123");
            assert_eq!("123", ty.pr_str());
        }
        {
            let ty = MalType::completed_list(vec![MalType::int("123"), MalType::int("456")]);
            assert_eq!("(123 456)", ty.pr_str());
        }
        {
            let ty = MalType::completed_list(vec![
                MalType::string("+"),
                MalType::int("2"),
                MalType::completed_list(vec![
                    MalType::string("*"),
                    MalType::int("3"),
                    MalType::int("4"),
                ]),
            ]);
            assert_eq!("(+ 2 (* 3 4))", ty.pr_str());
        }
    }
}
