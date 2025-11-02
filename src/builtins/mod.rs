use crate::commands::Command;

mod exit;
use exit::Exit;

pub fn get_builtin(cmd: &str) -> Option<impl Command> {
    match cmd {
        "exit" => Some(Exit::new()),
        _ => None,
    }
}
