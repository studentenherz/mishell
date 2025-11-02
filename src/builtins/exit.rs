use std::process::exit;

use crate::args::Args;
use crate::commands::*;

pub struct Exit;

impl Exit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Exit {
    fn eval(&self, args: Args) -> CommandReturnType {
        if let Some(exit_code) = args.args.iter().nth(1) {
            if let Ok(exit_code) = exit_code.parse::<i32>() {
                exit(exit_code);
            }
        }

        exit(0);
    }
}
