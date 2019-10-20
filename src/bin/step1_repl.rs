use std::io::{stdin, stdout, Write};
use mal_rs::quux::{MalResult, MalErr};
use mal_rs::quux::reader::read_str;
use mal_rs::quux::types::MalType;
use mal_rs::quux::printer::Printer;

pub fn read(s: &str) -> MalResult {
    read_str(s)
}

pub fn eval(ast: &MalType, _env: &str) -> MalResult {
    // TODO
    Ok(ast.clone())
}

pub fn print(exp: &MalType) -> String {
    exp.pr_str()
}

pub fn rep(s: &str) -> Result<String, MalErr> {
    read(s).and_then(|ty| eval(&ty, "")).map(|ty| print(&ty))
}

pub fn main() {
    loop {
        print!("user> ");
        stdout().flush().ok();

        let mut line = String::new();
        stdin().read_line(&mut line).ok();

        match rep(&line) {
            Ok(r) => println!("{}", r.to_string()),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rep;

    #[test]
    fn works() {
        {
            let result = &rep("123").unwrap();
            assert_eq!("123", result);
        }
        {
            let result = &rep("123 ").unwrap();
            assert_eq!("123", result);
        }
        {
            let result = &rep("abc").unwrap();
            assert_eq!("abc", result);
        }
        {
            let result = &rep("abc ").unwrap();
            assert_eq!("abc", result);
        }
        {
            let result = &rep("(123 456)").unwrap();
            assert_eq!("(123 456)", result);
        }
        {
            let result = &rep("( 123 456 789 )").unwrap();
            assert_eq!("(123 456 789)", result);
        }
        {
            let result = &rep("( + 2 (* 3 4) )").unwrap();
            assert_eq!("(+ 2 (* 3 4))", result);
        }
    }
}
