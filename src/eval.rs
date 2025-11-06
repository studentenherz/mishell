use std::process;

use crate::args::Args;
use crate::locate::{locate, LocatedCommand};

pub fn eval(args: Args) {
    for command_args in args.commands {
        match locate(command_args.command()) {
            LocatedCommand::Builtin(cmd) => {
                cmd.eval(command_args);
            }
            LocatedCommand::Executable(_) => {
                let mut command = process::Command::new(command_args.command());
                if let Some(file) = command_args.get_stdin_file() {
                    command.stdin(file);
                }
                if let Some(file) = command_args.get_stdout_file() {
                    command.stdout(file);
                }
                if let Some(file) = command_args.get_stderr_file() {
                    command.stderr(file);
                }
                command.args(&command_args.args[1..]);

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
                eprintln!("{}: command not found", command_args.command());
            }
        }
    }
}
