use anyhow::{anyhow, Context, Result};
use std::{
    env,
    fs::File,
    io::{stdin, stdout, Read, Write},
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
        println!("usage: rlox [option] ...");
        println!("\nLox interpreter from Crafting Intpreters book, written in Rust.");
        println!("\nOptions:");
        println!("-h, --help\t: print this menu");
        println!("\nArguments:");
        println!("File\t\t: read and execute script file");
        println!("_\t\t: run repl");
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
    println!("{:?}", source);
    Ok(())
}

// #[derive(Debug)]
// enum ArgsErrors {
//     UnknownArg { input: String },
//     UnknownArgs { inputs: Vec<String> },
// }

// impl std::fmt::Display for ArgsErrors {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::UnknownArg { input } => write!(f, "Unknown arg: {:?}", input),
//             Self::UnknownArgs { inputs } => write!(f, "Unknown args: {:?}", inputs),
//         }
//     }
// }

// fn show_full_menu() -> Result<()> {
//     Ok(())
// }

// fn show_short_menu() -> Result<()> {
//     Ok(())
// }
// fn check_args(arg: Vec<&str>) -> Result<()> {
//     return match arg {
//         "-h" | "--help" => show_full_menu(),
//         _ => {
//             show_short_menu()?;
//             Err(anyhow!(ArgsErrors::UnknownArg { input: arg.into() }))
//         }
//     };
// }
