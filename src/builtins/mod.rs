use crate::commands::Command;

mod echo;
mod exit;

use echo::Echo;
use exit::Exit;

pub fn get_builtin(cmd: &str) -> Option<Box<dyn Command>> {
    match cmd {
        "exit" => Some(Box::new(Exit::new())),
        "echo" => Some(Box::new(Echo::new())),
        _ => None,
    }
}
