use std::io::{Read, Write};

use rustyline::history::History as HistoryTrait;

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct History;

impl History {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for History {
    fn eval(
        &self,
        shell_ctx: &mut Shell,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        mut stdout: Box<dyn Write>,
        mut stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        let limit: Option<usize> = match args.args.iter().nth(1) {
            Some(val) => match val.as_str().parse() {
                Ok(value) => Some(value),
                Err(_) => {
                    let _ = stderr.write_all(b"history: numeric argument required\n");
                    let _ = stderr.flush();
                    return CommandReturnType {};
                }
            },
            None => None,
        };

        let skip = if let Some(limit) = limit {
            usize::saturating_sub(shell_ctx.rl.history().len(), limit)
        } else {
            0
        };

        let mut output = String::new();
        for (i, command) in shell_ctx.rl.history().iter().skip(skip).enumerate() {
            output.push_str(&format!("{:>5}  {}\n", skip + i + 1, command));
        }

        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();

        CommandReturnType {}
    }
}
