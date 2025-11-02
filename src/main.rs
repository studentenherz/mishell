use std::io::{self, Write};

mod args;
mod builtins;
mod commands;
mod eval;

use args::Args;
use eval::eval;

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn parse_args() -> Args {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Args::new(&input)
}

fn main() {
    loop {
        print_prompt();
        let args = parse_args();
        eval(args);
    }
}
