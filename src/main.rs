use anyhow::Result;
use std::{
    env,
    io::{stdin, stdout, Write},
};
use termion::input::TermRead;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_repl(),
        1 => run_w_arg(&args[0]),
        _ => todo!(),
    }
}

fn run_repl() -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write_all(b"password: ")?;

    stdout.flush()?;

    let pass = stdin.read_passwd(&mut stdout);

    if let Ok(Some(pass)) = pass {
        stdout.write_all(b"\n")?;
        stdout.write_all(pass.as_bytes())?;
        stdout.write_all(b"\n")?;
    } else {
        stdout.write_all(b"Error\n")?;
    }
    Ok(())
}

fn run_w_arg(arg: &str) -> Result<()> {
    if arg == "-h" || arg == "--help" {
        println!("");
    }

    println!("Cli Arg: {}", arg);

    Ok(())
}
