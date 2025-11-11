use crate::commands::Builtin;

mod cd;
mod echo;
mod exit;
pub mod history;
mod pwd;
mod type_;

use cd::Cd;
use echo::Echo;
use exit::Exit;
use history::History;
use pwd::Pwd;
use type_::Type;

pub const BUILTIN_COMANDS: [&str; 6] = ["cd", "exit", "echo", "type", "pwd", "history"];

pub fn get_builtin(cmd: &str) -> Option<Box<dyn Builtin>> {
    match cmd {
        "cd" => Some(Box::new(Cd::new())),
        "exit" => Some(Box::new(Exit::new())),
        "echo" => Some(Box::new(Echo::new())),
        "type" => Some(Box::new(Type::new())),
        "pwd" => Some(Box::new(Pwd::new())),
        "history" => Some(Box::new(History::new())),
        _ => None,
    }
}
