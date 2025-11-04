use crate::args::Args;
use crate::commands::*;
use std::io::Write;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Echo {
    fn eval(&self, args: Args) -> CommandReturnType {
        let mut stdout = args.stdout();
        let output = format!("{}\n", args.args[1..].join(" "));

        stdout.write_all(&output.as_bytes()).unwrap();
        stdout.flush().unwrap();

        CommandReturnType {}
    }
}
