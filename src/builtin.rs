use std::env;
use std::process;

use crate::system;

type BuiltinOption = Option<Box<dyn Fn(Vec<&str>) -> Result<(), String>>>;

pub fn get_builtin(keyword: &str) -> BuiltinOption {
    let f = match keyword {
        "cd" => cd,
        "echo" => echo,
        "exit" => exit,
        "pwd" => pwd,
        "type" => type_,
        _ => return None,
    };

    Some(Box::new(f))
}

pub fn cd(args: Vec<&str>) -> Result<(), String> {
    let path = args.first().map_or_else(system::home, |&s| {
        if s.starts_with('~') {
            s.replacen('~', &system::home(), 1)
        } else {
            s.into()
        }
    });

    if env::set_current_dir(&path).is_err() {
        Err(format!("{path}: No such file or directory"))
    } else {
        Ok(())
    }
}

pub fn echo(args: Vec<&str>) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}

pub fn exit(args: Vec<&str>) -> Result<(), String> {
    let exit_code = args
        .first()
        .map(|arg| arg.parse::<i32>().unwrap_or(0))
        .unwrap_or(0);

    process::exit(exit_code);
}

pub fn pwd(_args: Vec<&str>) -> Result<(), String> {
    if let Ok(path) = env::current_dir() {
        println!("{}", path.display());
        Ok(())
    } else {
        Err("failed to read current directory".into())
    }
}

pub fn type_(args: Vec<&str>) -> Result<(), String> {
    let keyword = args[0];

    if get_builtin(keyword).is_some() {
        println!("{keyword} is a shell builtin");
    } else if let Some(path) = system::find_on_path(keyword) {
        println!("{keyword} is {}", path.display());
    } else {
        println!("{keyword} not found");
    }

    Ok(())
}
