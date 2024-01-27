use anyhow::{Context, Ok, Result};
use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_repl(),
        1 => run_file(&args[0]),
        _ => todo!(),
    }
}

fn run_repl() -> Result<()> {
    let mut stdout = stdout();
    let mut stdin = stdin().lock();

    let mut line = String::new();

    loop {
        print!(">> ");
        stdout.flush()?;

        let _by = stdin.read_line(&mut line)?;
        let line_in = line.trim();

        if line_in == "exit" {
            break Ok(());
        }
        run(line_in.into())?;
        line.clear();
    }
}

fn run_file(arg: &str) -> Result<()> {
    if arg == "-h" || arg == "--help" {
        println!("");
        return Ok(());
    }

    let mut file = File::open(arg).with_context(|| format!("Could not open file {}", arg))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    run(buf)?;
    Ok(())
}

fn run(source: String) -> Result<()> {
    println!("{}", source);
    Ok(())
}
