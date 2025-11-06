use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    history::FileHistory,
    validate::Validator,
    Context, Editor, Helper, Result,
};
use std::io;

use crate::builtins::BUILTIN_COMANDS;
use crate::eval::eval;
use crate::trie::Trie;
use crate::{args::Args, locate::get_executables_names};

struct ShellHelper {
    commands: Trie,
}

impl ShellHelper {
    pub fn new() -> Self {
        let mut commands = Trie::new();

        for cmd in BUILTIN_COMANDS {
            commands.insert(cmd);
        }

        for exe in get_executables_names() {
            commands.insert(&exe);
        }

        Self { commands }
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        let command_start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let current_word = &line[command_start..pos];

        let matches = self
            .commands
            .matches(current_word)
            .into_iter()
            .map(|name| Pair {
                display: name.clone(),
                replacement: name,
            })
            .collect();

        Ok((command_start, matches))
    }

    fn update(
        &self,
        line: &mut rustyline::line_buffer::LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut rustyline::Changeset,
    ) {
        let end = line.pos();
        let with_space = format!("{} ", elected);
        line.replace(start..end, &with_space, cl);
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}

impl Helper for ShellHelper {}

pub struct Shell {
    rl: Editor<ShellHelper, FileHistory>,
}

impl Shell {
    pub fn new() -> Self {
        let mut rl = Editor::<ShellHelper, FileHistory>::new().unwrap();
        rl.set_helper(Some(ShellHelper::new()));

        Self { rl }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            let readline = &self.rl.readline("$ ");

            match readline {
                Ok(line) => {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        continue;
                    }

                    let _ = &self.rl.add_history_entry(line_trimmed).unwrap();

                    let args = Args::new(&line);

                    if args.command() == "exit" {
                        break;
                    }
                    eval(args);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error reading line: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }
}
