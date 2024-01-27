use anyhow::{Context, Result};
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
    let stdin = stdin();

    let mut buf = String::new();

    loop {
        print!(">> ");
        stdout.flush()?;

        let _by = stdin.read_line(&mut buf)?;
        // TODO: should I read the bytes read?

        let line = buf.trim();

        if line == "exit" {
            break Ok(());
        }

        run(line.into())?;
        buf.clear();
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
    // TODO: add tokenizer here
    println!("{}", source);
    Ok(())
}
