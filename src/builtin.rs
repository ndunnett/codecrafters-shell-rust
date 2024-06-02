use std::process;

use crate::system;

type BuiltinOption = Option<Box<dyn Fn(Vec<&str>) -> Result<(), String>>>;

pub fn get_builtin(keyword: &str) -> BuiltinOption {
    let f = match keyword {
        "cd" => cd,
        "echo" => echo,
        "exit" => exit,
        "type" => type_,
        _ => return None,
    };

    Some(Box::new(f))
}

pub fn cd(_args: Vec<&str>) -> Result<(), String> {
    unimplemented!()
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
