use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process::{self, ChildStdout, Stdio};

use crate::args::{Args, StdioConfig};
use crate::locate::{locate, LocatedCommand};
use crate::shell::ShellError;

enum Stdin {
    File(File),
    Std(io::Stdin),
}

impl Stdin {
    pub fn new(stdio_config: &StdioConfig) -> io::Result<Self> {
        Ok(match stdio_config {
            StdioConfig::File { path, append: _ } => {
                Self::File(fs::OpenOptions::new().read(true).open(path)?)
            }
            _ => Self::Std(io::stdin()),
        })
    }

    fn take(self) -> Box<dyn Read> {
        match self {
            Self::Std(stdin) => Box::new(stdin),
            Self::File(file) => Box::new(file),
        }
    }
}

enum Stdout {
    File(File),
    Std(io::Stdout),
    PipedBuiltin((io::PipeReader, io::PipeWriter)),
    PipedExternal,
}

impl From<Stdout> for Stdio {
    fn from(value: Stdout) -> Self {
        match value {
            Stdout::File(file) => Stdio::from(file),
            Stdout::Std(stdin) => Stdio::from(stdin),
            Stdout::PipedBuiltin((_, pipe_writer)) => Stdio::from(pipe_writer),
            Stdout::PipedExternal => Stdio::piped(),
        }
    }
}

impl Stdout {
    pub fn new(stdio_config: &StdioConfig, builtin: bool) -> io::Result<Self> {
        Ok(match stdio_config {
            StdioConfig::File { path, append } => Self::File(
                fs::OpenOptions::new()
                    .write(true)
                    .append(*append)
                    .open(path)?,
            ),
            StdioConfig::Std => Self::Std(io::stdout()),
            StdioConfig::Piped if builtin => Self::PipedBuiltin(io::pipe().unwrap()),
            _ => Self::PipedExternal,
        })
    }

    fn unwrap(self) -> Box<dyn Write> {
        match self {
            Self::Std(stdout) => Box::new(stdout),
            Self::File(file) => Box::new(file),
            Self::PipedBuiltin((_, pipe_writer)) => Box::new(pipe_writer),
            Self::PipedExternal => unreachable!(),
        }
    }
}

enum Stderr {
    File(File),
    Std(io::Stderr),
}

impl From<Stderr> for Stdio {
    fn from(value: Stderr) -> Self {
        match value {
            Stderr::File(file) => Stdio::from(file),
            Stderr::Std(stdin) => Stdio::from(stdin),
        }
    }
}

impl Stderr {
    pub fn new(stdio_config: &StdioConfig) -> io::Result<Self> {
        Ok(match stdio_config {
            StdioConfig::File { path, append } => Self::File(
                fs::OpenOptions::new()
                    .write(true)
                    .append(*append)
                    .open(path)?,
            ),
            _ => Self::Std(io::stderr()),
        })
    }

    fn unwrap(self) -> Box<dyn Write> {
        match self {
            Self::Std(stderr) => Box::new(stderr),
            Self::File(file) => Box::new(file),
        }
    }
}

enum PrevPipedStdout {
    External(ChildStdout),
    Builtin(io::PipeReader),
    None,
}

impl PrevPipedStdout {
    pub fn unwrap(self) -> Box<dyn Read> {
        match self {
            Self::External(stdout) => Box::new(stdout),
            Self::Builtin(pipe_reader) => Box::new(pipe_reader),
            Self::None => panic!("Tried to unwrap a None variant"),
        }
    }
}

pub fn eval(args: Args) -> Result<(), ShellError> {
    let mut prev_stdout = PrevPipedStdout::None;
    let mut handles = vec![];

    for command_args in args.commands {
        let curr_stdin = Stdin::new(&command_args.stdin)?;
        let curr_stderr = Stderr::new(&command_args.stderr)?;

        match locate(command_args.command()) {
            LocatedCommand::Builtin(cmd) => {
                let curr_stdout = Stdout::new(&command_args.stdout, true)?;
                let this_stdin: Box<dyn Read> = match curr_stdin {
                    Stdin::Std(_)
                        if matches!(
                            prev_stdout,
                            PrevPipedStdout::Builtin(_) | PrevPipedStdout::External(_)
                        ) =>
                    {
                        Box::new(prev_stdout.unwrap())
                    }
                    _ => curr_stdin.take(),
                };

                match curr_stdout {
                    Stdout::PipedBuiltin((pipe_reader, pipe_writer)) => {
                        cmd.eval(
                            command_args,
                            this_stdin,
                            Box::new(pipe_writer),
                            curr_stderr.unwrap(),
                        );

                        prev_stdout = PrevPipedStdout::Builtin(pipe_reader);
                    }
                    _ => {
                        cmd.eval(
                            command_args,
                            this_stdin,
                            curr_stdout.unwrap(),
                            curr_stderr.unwrap(),
                        );

                        prev_stdout = PrevPipedStdout::None;
                    }
                }
            }
            LocatedCommand::Executable(_) => {
                let mut command = process::Command::new(command_args.command());
                let curr_stdout = Stdout::new(&command_args.stdout, false)?;
                let is_piped = matches!(curr_stdout, Stdout::PipedExternal);
                match prev_stdout {
                    PrevPipedStdout::External(stdout) => {
                        command.stdin(stdout);
                    }
                    PrevPipedStdout::Builtin(pipe_reader) => {
                        command.stdin(pipe_reader);
                    }
                    PrevPipedStdout::None => {}
                };
                command.stdout(curr_stdout);
                command.stderr(curr_stderr);
                command.args(&command_args.args[1..]);

                match command.spawn() {
                    Ok(mut child) => {
                        if is_piped {
                            prev_stdout = PrevPipedStdout::External(child.stdout.take().unwrap());
                        } else {
                            prev_stdout = PrevPipedStdout::None;
                        }

                        handles.push(child);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        prev_stdout = PrevPipedStdout::None;
                    }
                }
            }
            LocatedCommand::Unrecognized => {
                eprintln!("{}: command not found", command_args.command());
            }
        }
    }

    for mut handle in handles.into_iter() {
        let _ = handle.wait();
    }

    Ok(())
}
