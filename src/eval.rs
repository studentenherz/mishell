use std::process;

use crate::args::Args;
use crate::locate::{locate, LocatedCommand};

pub fn eval(args: Args) {
    match locate(args.command()) {
        LocatedCommand::Builtin(cmd) => {
            cmd.eval(args);
        }
        LocatedCommand::Executable(_) => {
            match process::Command::new(args.command())
                .args(&args.args[1..])
                .spawn()
            {
                Ok(mut child) => {
                    match child.wait() {
                        Ok(_exit_status) => {
                            //
                        }
                        Err(err) => {
                            eprintln!("{}", err)
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
        LocatedCommand::Unrecognized => {
            println!("{}: command not found", args.command());
        }
    }
}
