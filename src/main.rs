#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        let result = match parts.next() {
            Some("cd") => unimplemented!(),
            Some("echo") => unimplemented!(),
            Some("exit") => exit(
                parts
                    .next()
                    .map(|arg| arg.parse::<i32>().unwrap_or(0))
                    .unwrap_or(0),
            ),
            Some(keyword) => Err(format!("{keyword}: command not found")),
            None => Ok(()),
        };

        if let Err(e) = result {
            eprintln!("{}", e);
        }
    }
}
