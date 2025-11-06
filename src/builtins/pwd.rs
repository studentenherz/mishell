use std::env;

use crate::args::CommandArgs;
use crate::commands::*;

pub struct Pwd;

impl Pwd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Pwd {
    fn eval(&self, args: CommandArgs) -> CommandReturnType {
        if let Ok(cwd) = env::current_dir() {
            let (_stdin, mut stdout, _stderr) = args.stdio();
            let output = format!("{}\n", cwd.display());
            stdout.write_all(output.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }

        CommandReturnType {}
    }
}
