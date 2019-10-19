use regex::{Captures, Regex};

#[derive(Debug)]
pub struct Reader {
    pub tokens: Vec<String>,
    pub pos: usize,
}

pub fn read_str(expr: &str) -> () {
    let tokens = tokenize(expr);
    read_form(Reader::new(tokens));
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

fn read_form(reader: Reader) -> () {
    unimplemented!()
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

    use crate::quux::reader::{Reader, ReaderOps};

    #[test]
    fn it_works() {
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
}
