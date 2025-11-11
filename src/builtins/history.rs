use std::io::{self, Read, Write};
use std::usize;

use rustyline::history::History as HistoryTrait;

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct History;

impl History {
    pub fn new() -> Self {
        Self {}
    }

    fn _save_history<'a>(
        path: &str,
        history: impl Iterator<Item = &'a String>,
        append: bool,
    ) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .append(append)
            .truncate(!append)
            .write(true)
            .create(true)
            .open(path)?;

        for line in history {
            file.write_all(format!("{}\n", line).as_bytes())?;
        }

        file.write_all(b"\n")?;
        file.flush()?;

        Ok(())
    }

    pub fn save(shell_ctx: &mut Shell, path: &str) -> io::Result<()> {
        let history = shell_ctx.rl.history().iter();
        Self::_save_history(path, history, false)
    }

    pub fn append(shell_ctx: &mut Shell, path: &str) -> io::Result<()> {
        let history = shell_ctx.rl.history().iter();
        Self::_save_history(path, history, true)
    }
}

#[derive(Default)]
struct Args {
    pub limit: Option<usize>,
    pub read: Option<String>,
    pub write: Option<String>,
    pub append: Option<String>,
}

struct ParseError {
    pub message: String,
}

impl ParseError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Args {
    fn parse(args: CommandArgs) -> Result<Self, ParseError> {
        let mut parsed = Self::default();
        let mut iter = args.args.iter().skip(1);
        let mut positional_index = 0usize;

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-r" => {
                    if let Some(path) = iter.next() {
                        parsed.read = Some(path.clone());
                        continue;
                    }

                    return Err(ParseError::new("Expected file name"));
                }
                "-w" => {
                    if let Some(path) = iter.next() {
                        parsed.write = Some(path.clone());
                        continue;
                    }

                    return Err(ParseError::new("Expected file name"));
                }
                "-a" => {
                    if let Some(path) = iter.next() {
                        parsed.append = Some(path.clone());
                        continue;
                    }

                    return Err(ParseError::new("Expected file name"));
                }
                _ => {
                    positional_index += 1;
                    match positional_index {
                        1 => match &str::parse::<usize>(arg) {
                            Ok(limit) => {
                                parsed.limit = Some(*limit);
                            }
                            Err(_) => {
                                return Err(ParseError::new("numeric argument required"));
                            }
                        },
                        _ => {
                            // Should I return an error here to say more arguments than expected
                            // were found?
                        }
                    }
                }
            }
        }

        Ok(parsed)
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
        let Args {
            limit,
            read,
            write,
            append,
        } = match Args::parse(args) {
            Ok(parsed) => parsed,
            Err(err) => {
                let _ = stderr.write_all(format!("history: {}\n", err.message).as_bytes());
                let _ = stderr.flush();
                return CommandReturnType {};
            }
        };

        if let Some(path) = read {
            if let Err(err) = shell_ctx.rl.load_history(&path) {
                let _ = stderr.write_all(format!("history: {}\n", err).as_bytes());
                let _ = stderr.flush();
            }
            return CommandReturnType {};
        }

        if let Some(path) = write {
            if let Err(err) = Self::save(shell_ctx, &path) {
                let _ = stderr.write_all(format!("history: {}\n", err).as_bytes());
                let _ = stderr.flush();
            }
            return CommandReturnType {};
        }

        if let Some(path) = append {
            if let Err(err) = Self::append(shell_ctx, &path) {
                let _ = stderr.write_all(format!("history: {}\n", err).as_bytes());
                let _ = stderr.flush();
            }
            return CommandReturnType {};
        }

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
