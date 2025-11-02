use std::env;
use std::path::Path;

use crate::args::Args;
use crate::commands::*;

pub struct Cd;

impl Cd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Cd {
    fn eval(&self, args: Args) -> CommandReturnType {
        if let Some(arg1) = args.args.iter().nth(1) {
            let target_dir = Path::new(arg1);
            if matches!(target_dir.try_exists(), Ok(true)) && target_dir.is_dir() {
                if env::set_current_dir(target_dir).is_ok() {
                    return CommandReturnType {};
                }
            }

            println!("cd: {}: No such file or directory", arg1);
        }

        CommandReturnType {}
    }
}
