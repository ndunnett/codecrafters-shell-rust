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

        if let Some(keyword) = parts.next() {
            if let Some(f) = builtin::get_builtin(keyword) {
                let _ = f(parts.collect());
            } else {
                eprintln!("{keyword}: command not found");
            }
        }
    }
}
