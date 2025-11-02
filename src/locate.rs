use std::env;
use std::fs;
use std::path::PathBuf;

use crate::builtins::get_builtin;
use crate::commands::*;

pub enum LocatedCommand {
    Builtin(Box<dyn Command>),
    Executable(PathBuf),
    Unrecognized,
}

pub fn locate(cmd: &str) -> LocatedCommand {
    if let Some(command) = get_builtin(cmd) {
        return LocatedCommand::Builtin(command);
    }

    if let Some(path) = env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            let executable_path = dir.join(cmd);
            if matches!(fs::exists(&executable_path), Ok(true)) {
                return LocatedCommand::Executable(executable_path);
            }
        }
    }

    LocatedCommand::Unrecognized
}
