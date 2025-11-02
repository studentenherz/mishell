use crate::args::Args;

pub struct CommandReturnType;

pub trait Command {
    fn eval(&self, args: Args) -> CommandReturnType;
}
