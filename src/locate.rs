use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::builtins::get_builtin;
use crate::commands::*;

pub enum LocatedCommand {
    Builtin(Box<dyn Builtin>),
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
            if matches!(executable_path.try_exists(), Ok(true)) {
                if let Ok(metadata) = fs::metadata(&executable_path) {
                    if metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0 {
                        return LocatedCommand::Executable(executable_path);
                    }
                }
            }
        }
    }

    LocatedCommand::Unrecognized
}
