use crate::commands::Command;

mod echo;
mod exit;
mod pwd;
mod type_;

use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use type_::Type;

pub fn get_builtin(cmd: &str) -> Option<Box<dyn Command>> {
    match cmd {
        "exit" => Some(Box::new(Exit::new())),
        "echo" => Some(Box::new(Echo::new())),
        "type" => Some(Box::new(Type::new())),
        "pwd" => Some(Box::new(Pwd::new())),
        _ => None,
    }
}
