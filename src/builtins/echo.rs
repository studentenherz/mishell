use std::io::Write;

use crate::args::CommandArgs;
use crate::commands::*;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Echo {
    fn eval(&self, args: CommandArgs) -> CommandReturnType {
        let (_stdin, mut stdout, _stderr) = args.stdio();
        let output = format!("{}\n", args.args[1..].join(" "));

        stdout.write_all(&output.as_bytes()).unwrap();
        stdout.flush().unwrap();

        CommandReturnType {}
    }
}
