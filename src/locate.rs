use crate::builtins::get_builtin;
use crate::commands::*;

pub enum LocatedCommand {
    Builtin(Box<dyn Command>),
    Unrecognized,
}

pub fn locate(cmd: &str) -> LocatedCommand {
    if let Some(command) = get_builtin(cmd) {
        return LocatedCommand::Builtin(command);
    }

    LocatedCommand::Unrecognized
}
