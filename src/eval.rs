use crate::args::Args;
use crate::locate::{locate, LocatedCommand};

pub fn eval(args: Args) {
    match locate(args.command()) {
        LocatedCommand::Builtin(cmd) => {
            cmd.eval(args);
        }
        LocatedCommand::Unrecognized => {
            println!("{}: command not found", args.command());
        }
    }
}
