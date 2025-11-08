use crate::args::CommandArgs;
use crate::commands::*;
use crate::locate::{locate, LocatedCommand};

pub struct Type;

impl Type {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Type {
    fn eval(
        &self,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        mut stdout: Box<dyn Write>,
        _stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        if let Some(cmd) = args.args.iter().nth(1) {
            let output = match locate(cmd) {
                LocatedCommand::Builtin(_) => {
                    format!("{} is a shell builtin\n", cmd)
                }
                LocatedCommand::Executable(path) => {
                    format!("{} is {}\n", cmd, path.display())
                }
                LocatedCommand::Unrecognized => {
                    format!("{}: not found\n", cmd)
                }
            };
            stdout.write_all(output.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }

        CommandReturnType {}
    }
}
