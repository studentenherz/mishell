#[derive(Clone)]
pub struct OutRedirectOptions {
    pub filename: String,
    pub append: bool,
}

#[derive(Clone)]
pub struct InputRedirectOptions {
    pub filename: String,
}

pub struct CommandArgs {
    pub args: Vec<String>,
    redirect_stdin: Option<InputRedirectOptions>,
    redirect_stdout: Option<OutRedirectOptions>,
    redirect_stderr: Option<OutRedirectOptions>,
}

impl CommandArgs {
    pub fn command(&self) -> &str {
        &self.args[0]
    }

    pub fn get_stdin_file(&self) -> Option<std::fs::File> {
        self.redirect_stdin
            .clone()
            .map(|InputRedirectOptions { filename }| {
                std::fs::File::options()
                    .create(true)
                    .read(true)
                    .open(filename)
                    .expect("shell: couldn't create file")
            })
    }

    pub fn get_stdout_file(&self) -> Option<std::fs::File> {
        self.redirect_stdout
            .clone()
            .map(|OutRedirectOptions { filename, append }| {
                std::fs::File::options()
                    .append(append)
                    .create(true)
                    .write(true)
                    .open(filename)
                    .expect("shell: couldn't create file")
            })
    }

    pub fn get_stderr_file(&self) -> Option<std::fs::File> {
        self.redirect_stderr
            .clone()
            .map(|OutRedirectOptions { filename, append }| {
                std::fs::File::options()
                    .append(append)
                    .create(true)
                    .write(true)
                    .open(filename)
                    .expect("shell: couldn't create file")
            })
    }

    fn stdin(&self) -> Box<dyn std::io::Read> {
        match self.get_stdin_file() {
            Some(file) => Box::new(file),
            None => Box::new(std::io::stdin()),
        }
    }

    fn stdout(&self) -> Box<dyn std::io::Write> {
        match self.get_stdout_file() {
            Some(file) => Box::new(file),
            None => Box::new(std::io::stdout()),
        }
    }

    fn stderr(&self) -> Box<dyn std::io::Write> {
        match self.get_stderr_file() {
            Some(file) => Box::new(file),
            None => Box::new(std::io::stderr()),
        }
    }

    pub fn stdio(
        &self,
    ) -> (
        Box<dyn std::io::Read>,
        Box<dyn std::io::Write>,
        Box<dyn std::io::Write>,
    ) {
        (self.stdin(), self.stdout(), self.stderr())
    }
}

pub struct Args {
    // This could probable be better with a more complex
    // structure like a DAG, but for now, only piping will
    // be implemented and it would be overkill.
    pub commands: Vec<CommandArgs>,
}

enum QuoteState {
    Single,
    Double,
    None,
}

impl Args {
    fn parse_args(input: &str) -> Vec<String> {
        let mut args = Vec::<String>::new();

        let mut state = QuoteState::None;
        let mut curr_arg = String::new();
        let mut escaped = false;

        for character in input.trim().chars() {
            match state {
                QuoteState::Single => {
                    if character == '\'' {
                        state = QuoteState::None;
                    } else {
                        curr_arg.push(character);
                    }
                }
                QuoteState::Double => match character {
                    _ if escaped => {
                        if !['"', '\\', '$', '`'].contains(&character) {
                            curr_arg.push('\\');
                        }
                        curr_arg.push(character);
                        escaped = false;
                    }
                    '\\' => {
                        escaped = true;
                    }
                    '"' => {
                        state = QuoteState::None;
                    }
                    _ => {
                        curr_arg.push(character);
                    }
                },
                QuoteState::None => match character {
                    _ if escaped => {
                        curr_arg.push(character);
                        escaped = false;
                    }
                    '\\' => {
                        escaped = true;
                    }
                    '\'' => {
                        state = QuoteState::Single;
                    }
                    '"' => {
                        state = QuoteState::Double;
                    }
                    character if character.is_whitespace() => {
                        let arg = curr_arg.trim();
                        if !arg.is_empty() {
                            args.push(arg.to_string());
                        }
                        curr_arg.clear();
                    }
                    _ => {
                        curr_arg.push(character);
                    }
                },
            }
        }

        args.push(curr_arg);
        args
    }

    pub fn new(input: &str) -> Self {
        let mut raw_args = Self::parse_args(input);
        raw_args.push("|".to_string());

        let mut commands = Vec::<CommandArgs>::new();
        let mut args = Vec::<String>::new();
        let mut redirect_stdin = None;
        let mut redirect_stdout = None;
        let mut redirect_stderr = None;

        let mut iter = raw_args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "1>" | ">" => {
                    redirect_stdout = Some(OutRedirectOptions {
                        filename: iter.next().expect("shell: parse error"),
                        append: false,
                    })
                }
                "1>>" | ">>" => {
                    redirect_stdout = Some(OutRedirectOptions {
                        filename: iter.next().expect("shell: parse error"),
                        append: true,
                    })
                }
                "2>" => {
                    redirect_stderr = Some(OutRedirectOptions {
                        filename: iter.next().expect("shell: parse error"),
                        append: false,
                    })
                }
                "2>>" => {
                    redirect_stderr = Some(OutRedirectOptions {
                        filename: iter.next().expect("shell: parse error"),
                        append: true,
                    })
                }
                "<" => {
                    redirect_stdin = Some(InputRedirectOptions {
                        filename: iter.next().expect("shell: parse error"),
                    })
                }
                "|" => {
                    commands.push(CommandArgs {
                        args: args.clone(),
                        redirect_stdin: redirect_stdin.clone(),
                        redirect_stdout: redirect_stdout.clone(),
                        redirect_stderr: redirect_stderr.clone(),
                    });

                    args.clear();
                    redirect_stdin = None;
                    redirect_stdout = None;
                    redirect_stderr = None;
                }
                _ => {
                    args.push(arg);
                }
            }
        }

        Self { commands }
    }
}
