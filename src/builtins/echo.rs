use crate::args::Args;
use crate::commands::*;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Echo {
    fn eval(&self, args: Args) -> CommandReturnType {
        println!("{}", args.args[1..].join(" "));

        CommandReturnType {}
    }
}
