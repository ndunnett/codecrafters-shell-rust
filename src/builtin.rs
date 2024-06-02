use std::process;

pub fn echo(args: Vec<&str>) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}

pub fn exit(args: Vec<&str>) -> ! {
    let exit_code = args
        .first()
        .map(|arg| arg.parse::<i32>().unwrap_or(0))
        .unwrap_or(0);

    process::exit(exit_code);
}
