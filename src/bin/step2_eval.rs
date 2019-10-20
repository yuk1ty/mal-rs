use mal_rs::quux::env::Env;
use mal_rs::quux::printer::Printer;
use mal_rs::quux::reader::read_str;
use mal_rs::quux::types::MalType;
use mal_rs::quux::{MalErr, MalResult};
use std::io::{stdin, stdout, Write};

pub fn read(s: &str) -> MalResult {
    read_str(s)
}

pub fn eval(ast: &MalType, _env: &Env) -> MalResult {
    // TODO
    Ok(ast.clone())
}

pub fn print(exp: &MalType) -> String {
    exp.pr_str()
}

pub fn rep(s: &str, env: &Env) -> Result<String, MalErr> {
    read(s).and_then(|ty| eval(&ty, env)).map(|ty| print(&ty))
}

pub fn main() {
    let env = Env::init();

    loop {
        print!("user> ");
        stdout().flush().ok();

        let mut line = String::new();
        stdin().read_line(&mut line).ok();

        match rep(&line, &env) {
            Ok(r) => println!("{}", r.to_string()),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rep;
    use mal_rs::quux::env::Env;

    #[test]
    fn works() {
        let env = Env::init();

        {
            let result = &rep("123", &env).unwrap();
            assert_eq!("123", result);
        }
        {
            let result = &rep("123 ", &env).unwrap();
            assert_eq!("123", result);
        }
        {
            let result = &rep("abc", &env).unwrap();
            assert_eq!("abc", result);
        }
        {
            let result = &rep("abc ", &env).unwrap();
            assert_eq!("abc", result);
        }
        {
            let result = &rep("(123 456)", &env).unwrap();
            assert_eq!("(123 456)", result);
        }
        {
            let result = &rep("( 123 456 789 )", &env).unwrap();
            assert_eq!("(123 456 789)", result);
        }
        {
            let result = &rep("( + 2 (* 3 4) )", &env).unwrap();
            assert_eq!("(+ 2 (* 3 4))", result);
        }
    }
}
