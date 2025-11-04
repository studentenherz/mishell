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
    fn eval(&self, args: Args) -> CommandReturnType {
        if let Ok(cwd) = env::current_dir() {
            let mut stdout = args.stdout();
            let output = format!("{}\n", cwd.display());
            stdout.write_all(output.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }

        CommandReturnType {}
    }
}
