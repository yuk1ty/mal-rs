use std::io::{stdin, stdout, Write};

pub fn read(s: &str) -> String {
    s.to_string()
}

pub fn eval(ast: &str, _env: &str) -> String {
    ast.to_string()
}

pub fn print(exp: &str) -> String {
    exp.to_string()
}

pub fn rep(s: &str) -> String {
    print(&eval(&read(s), ""))
}

pub fn main() {
    loop {
        print!("user> ");
        stdout().flush().ok();

        let mut line = String::new();
        stdin().read_line(&mut line).ok();
        println!("{}", rep(&line));
    }
}
