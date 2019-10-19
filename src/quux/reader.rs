use crate::quux::types::MalType;
use crate::quux::{raise_err, MalResult};
use regex::Regex;

#[derive(Debug)]
pub struct Reader {
    pub tokens: Vec<String>,
    pub pos: usize,
}

pub fn read_str(expr: &str) -> MalResult {
    let tokens = tokenize(expr);
    read_form(&mut Reader::new(tokens))
}

fn tokenize(expr: &str) -> Vec<String> {
    lazy_static! {
        static ref REX: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        )
        .unwrap();
    }

    let mut result = vec![];
    for cap in REX.captures_iter(expr) {
        if cap[1].starts_with(";") {
            continue;
        }
        result.push(String::from(&cap[1]));
    }
    result
}

fn read_form(reader: &mut Reader) -> MalResult {
    let maybe_expr = reader.peek();
    if maybe_expr.is_none() {
        return raise_err("There is no tokens");
    }

    match &maybe_expr.unwrap()[..] {
        "'" => {
            reader.next();
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("quote".to_string())]),
                Box::new(read_form(reader)?),
            ))
        }
        "`" => {
            reader.next();
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("quasiquote".to_string())]),
                Box::new(read_form(reader)?),
            ))
        }
        "~" => {
            reader.next();
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("unquote".to_string())]),
                Box::new(read_form(reader)?),
            ))
        }
        "~@" => {
            reader.next();
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("splice-unquote".to_string())]),
                Box::new(read_form(reader)?),
            ))
        }
        "^" => {
            reader.next();
            let meta = read_form(reader)?;
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("meta".to_string())]),
                Box::new(meta),
            ))
        }
        "@" => {
            reader.next();
            Ok(MalType::List(
                Box::new(vec![MalType::Symbol("deref".to_string())]),
                Box::new(read_form(reader)?),
            ))
        }
        ")" => raise_err("unexpected )"),
        "(" => read_list(reader, ")"),
        "]" => raise_err("unexpected ]"),
        "[" => read_list(reader, "]"),
        "}" => raise_err("unexpected }"),
        "{" => read_list(reader, "}"),
        _ => read_atom(reader),
    }
}

fn read_list(reader: &mut Reader, end: &str) -> MalResult {
    let mut ast: Vec<MalType> = Vec::new();
    reader.next();

    loop {
        let token = match reader.peek() {
            Some(t) => t,
            None => panic!("naito omoukedo"),
        };
        if &token == end {
            break;
        }
        ast.push(read_form(reader)?)
    }

    reader.next();

    match end {
        ")" => Ok(MalType::List(Box::new(ast), Box::new(MalType::Nil))),
        "]" => Ok(MalType::Vector(Box::new(ast), Box::new(MalType::Nil))),
        "}" => Ok(MalType::HashMap(Box::new(ast), Box::new(MalType::Nil))),
        _ => raise_err("unimplemented!"),
    }
}

fn read_atom(reader: &mut Reader) -> MalResult {
    lazy_static! {
        static ref INT_REX: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_REX: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let token = reader.next();
    match token {
        Some(t) => match &t[..] {
            "nil" => Ok(MalType::Nil),
            _ => {
                if INT_REX.is_match(&t) {
                    Ok(MalType::Int(t))
                } else if STR_REX.is_match(&t) {
                    Ok(MalType::String(t))
                } else {
                    Ok(MalType::Symbol(t))
                }
            }
        },
        None => raise_err("There is no token in read_atom"),
    }
}

pub trait ReaderOps {
    /// Returns the tokens at the current position and increments the position
    fn next(&mut self) -> Option<String>;

    /// Just returns the tokens at the current position
    fn peek(&self) -> Option<String>;
}

impl Reader {
    fn new(tokens: Vec<String>) -> Reader {
        Reader { tokens, pos: 0 }
    }
}

impl ReaderOps for Reader {
    fn next(&mut self) -> Option<String> {
        let found = self.tokens.get(self.pos).map(|s| s.to_string());
        self.pos += 1;
        found
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {

    use crate::quux::reader::{read_form, tokenize, Reader, ReaderOps};
    use crate::quux::types::MalType;

    #[test]
    fn next_peek_works() {
        let mut reader = Reader::new(
            vec!["t", "o", "k", "e", "n", "s"]
                .iter()
                .map(|c| c.to_string())
                .collect(),
        );
        assert_eq!("t".to_string(), reader.peek().unwrap());
        assert_eq!("t".to_string(), reader.next().unwrap());
        assert_eq!("o".to_string(), reader.peek().unwrap());
        assert_eq!("o".to_string(), reader.next().unwrap());
        assert_eq!("k".to_string(), reader.peek().unwrap());
        assert_eq!("k".to_string(), reader.next().unwrap());
        assert_eq!("e".to_string(), reader.peek().unwrap());
        assert_eq!("e".to_string(), reader.next().unwrap());
        assert_eq!("n".to_string(), reader.peek().unwrap());
        assert_eq!("n".to_string(), reader.next().unwrap());
        assert_eq!("s".to_string(), reader.peek().unwrap());
        assert_eq!("s".to_string(), reader.next().unwrap());
    }

    #[test]
    fn tokenize_works() {
        {
            let tokens = tokenize("abc bcd def");
            assert_eq!(vec!["abc", "bcd", "def"], tokens);
        }
        {
            let tokens = tokenize("abc; never read");
            assert_eq!(vec!["abc"], tokens);
        }
        {
            let tokens = tokenize("( 1 + 2 )");
            assert_eq!(vec!["(", "1", "+", "2", ")"], tokens);
        }
    }

    #[test]
    fn read_form_works() {
        {
            let ast = read_form(&mut Reader::new(tokenize("123")));
            assert_eq!(MalType::Int("123".to_string()), ast.unwrap());
        }
        {
            let ast = read_form(&mut Reader::new(tokenize("123 ")));
            assert_eq!(MalType::Int("123".to_string()), ast.unwrap());
        }
        {
            let ast = read_form(&mut Reader::new(tokenize("abc")));
            assert_eq!(MalType::Symbol("abc".to_string()), ast.unwrap());
        }
        {
            let ast = read_form(&mut Reader::new(tokenize("abc ")));
            assert_eq!(MalType::Symbol("abc".to_string()), ast.unwrap());
        }
        {
            let ast = read_form(&mut Reader::new(tokenize("(123 456)")));
            assert_eq!(
                MalType::List(
                    Box::new(vec![
                        MalType::Int("123".to_string()),
                        MalType::Int("456".to_string())
                    ]),
                    Box::new(MalType::Nil)
                ),
                ast.unwrap()
            );
        }
        {
            let ast = read_form(&mut Reader::new(tokenize("( + 2 (* 3 4) )")));
            assert_eq!(
                MalType::List(
                    Box::new(vec![
                        MalType::Symbol("+".to_string()),
                        MalType::Int("2".to_string()),
                        MalType::List(
                            Box::new(vec![
                                MalType::Symbol("*".to_string()),
                                MalType::Int("3".to_string()),
                                MalType::Int("4".to_string())
                            ]),
                            Box::new(MalType::Nil)
                        )
                    ]),
                    Box::new(MalType::Nil)
                ),
                ast.unwrap()
            );
        }
    }
}
