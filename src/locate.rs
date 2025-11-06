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

pub fn get_executables_names() -> Vec<String> {
    let mut executables = vec![];

    if let Some(path) = env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            if let Ok(read_dir) = dir.read_dir() {
                for dir_entry in read_dir {
                    if let Ok(file) = dir_entry {
                        if let Ok(metadata) = file.metadata() {
                            if metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0 {
                                executables.push(file.file_name().into_string().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }

    executables
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
