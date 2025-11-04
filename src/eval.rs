use std::process;

use crate::args::Args;
use crate::locate::{locate, LocatedCommand};

pub fn eval(args: Args) {
    match locate(args.command()) {
        LocatedCommand::Builtin(cmd) => {
            cmd.eval(args);
        }
        LocatedCommand::Executable(_) => {
            let mut command = process::Command::new(args.command());
            if let Some(file) = args.get_stdin_file() {
                command.stdin(file);
            }
            if let Some(file) = args.get_stdout_file() {
                command.stdout(file);
            }
            if let Some(file) = args.get_stderr_file() {
                command.stderr(file);
            }
            command.args(&args.args[1..]);

            match command.spawn() {
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
