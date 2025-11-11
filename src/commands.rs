pub use std::io::{Read, Write};

use crate::args::CommandArgs;
use crate::shell::Shell;

pub struct CommandReturnType;

pub trait Builtin {
    fn eval(
        &self,
        shell_ctx: &mut Shell,
        args: CommandArgs,
        stdin: Box<dyn Read>,
        stdout: Box<dyn Write>,
        stderr: Box<dyn Write>,
    ) -> CommandReturnType;
}
