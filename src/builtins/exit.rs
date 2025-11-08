use std::io::{Read, Write};

use std::process::exit;

use crate::args::CommandArgs;
use crate::commands::*;

pub struct Exit;

impl Exit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Exit {
    fn eval(
        &self,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        _stdout: Box<dyn Write>,
        _stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        if let Some(exit_code) = args.args.iter().nth(1) {
            if let Ok(exit_code) = exit_code.parse::<i32>() {
                exit(exit_code);
            }
        }

        exit(0);
    }
}
