use std::env;

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct Pwd;

impl Pwd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Pwd {
    fn eval(
        &self,
        _shell_ctx: &mut Shell,
        _args: CommandArgs,
        _stdin: Box<dyn Read>,
        mut stdout: Box<dyn Write>,
        _stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        if let Ok(cwd) = env::current_dir() {
            let output = format!("{}\n", cwd.display());
            stdout.write_all(output.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }

        CommandReturnType {}
    }
}
