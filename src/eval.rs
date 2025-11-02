use crate::args::Args;
use crate::builtins::get_builtin;

pub fn eval(args: Args) {
    let command = get_builtin(args.command());

    match command {
        Some(cmd) => {
            cmd.eval(args);
        }
        None => {
            println!("{}: command not found", args.command());
        }
    }
}
