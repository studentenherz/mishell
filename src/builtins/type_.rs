use crate::args::Args;
use crate::commands::*;
use crate::locate::{locate, LocatedCommand};

pub struct Type;

impl Type {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Type {
    fn eval(&self, args: Args) -> CommandReturnType {
        if let Some(cmd) = args.args.iter().nth(1) {
            match locate(cmd) {
                LocatedCommand::Builtin(_) => {
                    println!("{} is a shell builtin", cmd)
                }
                LocatedCommand::Executable(path) => {
                    println!("{} is {}", cmd, path.display());
                }
                LocatedCommand::Unrecognized => {
                    println!("{}: not found", cmd);
                }
            }
        }

        CommandReturnType {}
    }
}
