use std::io::{self, Write};
use std::process::Command;

mod builtin;
mod system;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        if let Some(keyword) = parts.next() {
            if let Some(f) = builtin::get_builtin(keyword) {
                if let Err(e) = f(parts.collect()) {
                    eprintln!("{e}");
                }
            } else if let Some(path) = system::find_on_path(keyword) {
                match Command::new(path).args(&parts.collect::<Vec<_>>()).spawn() {
                    Ok(mut c) => {
                        if let Err(e) = c.wait() {
                            eprintln!("{e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            } else {
                eprintln!("{keyword}: command not found");
            }
        }
    }
}
