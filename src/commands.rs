use crate::args::CommandArgs;

pub struct CommandReturnType;

pub trait Builtin {
    fn eval(&self, args: CommandArgs) -> CommandReturnType;
}
