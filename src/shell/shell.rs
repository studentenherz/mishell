use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    history::{FileHistory, History},
    validate::Validator,
    CompletionType, Config, Context, Editor, Helper, Result,
};
use std::{
    io::{self, Write},
    process::exit,
};

use crate::builtins::BUILTIN_COMANDS;
use crate::eval::eval;
use crate::trie::Trie;
use crate::{args::Args, locate::get_executables_names};

pub struct ShellHelper {
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
                replacement: format!("{} ", name),
            })
            .collect();

        Ok((command_start, matches))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
}

impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}

impl Helper for ShellHelper {}

pub struct Shell {
    pub rl: Editor<ShellHelper, FileHistory>,
    histfile: Option<String>,
    appended_count: usize,
}

impl Shell {
    pub fn new() -> Self {
        let config = Config::builder()
            .completion_type(CompletionType::List)
            .completion_prompt_limit(50)
            .history_ignore_dups(false)
            .unwrap()
            .build();

        let mut rl = Editor::with_config(config).unwrap();
        rl.set_helper(Some(ShellHelper::new()));

        let histfile = match std::env::var("HISTFILE") {
            Ok(path) => {
                let _ = rl.load_history(&path);
                Some(path)
            }
            _ => None,
        };

        Self {
            appended_count: rl.history().len(),
            rl,
            histfile,
        }
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

                    let _ = self.rl.add_history_entry(line_trimmed);

                    let args = Args::new(&line);

                    if let Err(err) = eval(self, args) {
                        eprintln!("mishell: {}", err.message);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    eprintln!("Error reading line: {:?}", err);
                    break;
                }
            }
        }

        self.exit(0);
        Ok(())
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
        file.flush()?;

        Ok(())
    }

    pub fn save(&mut self, path: &str) -> io::Result<()> {
        let history = self.rl.history().iter();
        Self::_save_history(path, history, false)
    }

    pub fn append(&mut self, path: &str) -> io::Result<()> {
        let history = self.rl.history().iter().skip(self.appended_count);
        self.appended_count = self.rl.history().len();
        Self::_save_history(path, history, true)
    }

    pub fn exit(&mut self, status: i32) {
        if let Some(path) = self.histfile.clone() {
            let _ = self.append(&path);
        }

        exit(status)
    }
}
