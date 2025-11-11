use std::io::{Read, Write};

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct Exit;

impl Exit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Exit {
    fn eval(
        &self,
        shell_ctx: &mut Shell,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        _stdout: Box<dyn Write>,
        _stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        let mut status = 0;
        if let Some(exit_code) = args.args.iter().nth(1) {
            if let Ok(exit_code) = exit_code.parse::<i32>() {
                status = exit_code;
            }
        }

        shell_ctx.exit(status);

        CommandReturnType {}
    }
}
