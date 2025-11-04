use crate::args::Args;

pub struct CommandReturnType;

pub trait Builtin {
    fn eval(&self, args: Args) -> CommandReturnType;
}
