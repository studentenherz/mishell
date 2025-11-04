use crate::args::Args;
use crate::commands::*;
use std::io::Write;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Echo {
    fn eval(&self, args: Args) -> CommandReturnType {
        let (_stdin, mut stdout, _stderr) = args.stdio();
        let output = format!("{}\n", args.args[1..].join(" "));

        stdout.write_all(&output.as_bytes()).unwrap();
        stdout.flush().unwrap();

        CommandReturnType {}
    }
}
