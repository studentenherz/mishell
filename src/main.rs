use std::io::{self, Write};

mod args;

use args::Args;

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn parse_args() -> Args {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Args::new(&input)
}

fn eval(args: Args) {
    println!("{}: command not found", args.command());
}

fn main() {
    loop {
        print_prompt();
        let args = parse_args();
        eval(args);
    }
}
