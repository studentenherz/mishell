use std::env;

use crate::args::Args;
use crate::commands::*;

pub struct Pwd;

impl Pwd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Pwd {
    fn eval(&self, _args: Args) -> CommandReturnType {
        if let Ok(cwd) = env::current_dir() {
            println!("{}", cwd.display());
        }

        CommandReturnType {}
    }
}
