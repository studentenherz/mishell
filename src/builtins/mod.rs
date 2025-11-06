use crate::commands::Builtin;

mod cd;
mod echo;
mod exit;
mod pwd;
mod type_;

use cd::Cd;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use type_::Type;

pub const BUILTIN_COMANDS: [&str; 5] = ["cd", "exit", "echo", "type", "pwd"];

pub fn get_builtin(cmd: &str) -> Option<Box<dyn Builtin>> {
    match cmd {
        "cd" => Some(Box::new(Cd::new())),
        "exit" => Some(Box::new(Exit::new())),
        "echo" => Some(Box::new(Echo::new())),
        "type" => Some(Box::new(Type::new())),
        "pwd" => Some(Box::new(Pwd::new())),
        _ => None,
    }
}
