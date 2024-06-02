#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        let result = match parts.next() {
            Some("cd") => unimplemented!(),
            Some("echo") => unimplemented!(),
            Some(keyword) => Err(format!("{keyword}: command not found")),
            None => Ok(()),
        };

        if let Err(e) = result {
            eprintln!("{}", e);
        }
    }
}
