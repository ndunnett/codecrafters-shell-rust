#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        let result = match parts.next() {
            Some("cd") => unimplemented!(),
            Some("echo") => builtin::echo(parts.collect()),
            Some("exit") => builtin::exit(parts.collect()),
            Some(keyword) => Err(format!("{keyword}: command not found")),
            None => Ok(()),
        };

        if let Err(e) = result {
            eprintln!("{}", e);
        }
    }
}
