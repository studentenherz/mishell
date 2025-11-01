use std::io::{self, Write};

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    command.trim().to_string()
}

fn eval(command: &str) {
    println!("{}: command not found", command);
}

fn main() {
    loop {
        print_prompt();
        let command = read_command();
        eval(&command);
    }
}
