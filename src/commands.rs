pub use std::io::{Read, Write};

use crate::args::CommandArgs;

pub struct CommandReturnType;

pub trait Builtin {
    fn eval(
        &self,
        args: CommandArgs,
        stdin: Box<dyn Read>,
        stdout: Box<dyn Write>,
        stderr: Box<dyn Write>,
    ) -> CommandReturnType;
}
