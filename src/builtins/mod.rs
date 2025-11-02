use crate::commands::Command;

mod echo;
mod exit;
mod type_;

use echo::Echo;
use exit::Exit;
use type_::Type;

pub fn get_builtin(cmd: &str) -> Option<Box<dyn Command>> {
    match cmd {
        "exit" => Some(Box::new(Exit::new())),
        "echo" => Some(Box::new(Echo::new())),
        "type" => Some(Box::new(Type::new())),
        _ => None,
    }
}
