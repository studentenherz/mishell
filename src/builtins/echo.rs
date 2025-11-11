use std::io::{Read, Write};

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Echo {
    fn eval(
        &self,
        _shell_ctx: &mut Shell,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        mut stdout: Box<dyn Write>,
        _stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        let output = format!("{}\n", args.args[1..].join(" "));

        stdout.write_all(&output.as_bytes()).unwrap();
        stdout.flush().unwrap();

        CommandReturnType {}
    }
}
