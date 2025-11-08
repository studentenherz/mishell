#[derive(Clone, Debug)]
pub enum StdioConfig {
    File { path: String, append: bool },
    Std,
    Piped,
}

#[derive(Debug)]
pub struct CommandArgs {
    pub args: Vec<String>,

    pub stdin: StdioConfig,
    pub stdout: StdioConfig,
    pub stderr: StdioConfig,
}

impl CommandArgs {
    pub fn command(&self) -> &str {
        &self.args[0]
    }
}

#[derive(Debug)]
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
        let raw_args = Self::parse_args(input);

        let mut commands = Vec::<CommandArgs>::new();
        let mut args = Vec::<String>::new();

        let mut stdin = StdioConfig::Std;
        let mut stdout = StdioConfig::Std;
        let mut stderr = StdioConfig::Std;
        let mut prev_piped = false;

        let mut iter = raw_args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "1>" | ">" => {
                    stdout = StdioConfig::File {
                        path: iter.next().expect("shell: parse error"),
                        append: false,
                    };
                }
                "1>>" | ">>" => {
                    stdout = StdioConfig::File {
                        path: iter.next().expect("shell: parse error"),
                        append: true,
                    };
                }
                "2>" => {
                    stderr = StdioConfig::File {
                        path: iter.next().expect("shell: parse error"),
                        append: false,
                    }
                }
                "2>>" => {
                    stderr = StdioConfig::File {
                        path: iter.next().expect("shell: parse error"),
                        append: true,
                    }
                }
                "<" => {
                    stdin = StdioConfig::File {
                        path: iter.next().expect("shell: parse error"),
                        append: false,
                    }
                }
                "|" => {
                    commands.push(CommandArgs {
                        args: args.clone(),
                        stdin: if matches!(stdin, StdioConfig::Std) && prev_piped {
                            StdioConfig::Piped
                        } else {
                            stdin.clone()
                        },
                        stderr: stderr.clone(),
                        stdout: if matches!(stdout, StdioConfig::Std) {
                            prev_piped = true;
                            StdioConfig::Piped
                        } else {
                            stdout.clone()
                        },
                    });

                    args.clear();
                    stdin = StdioConfig::Std;
                    stdout = StdioConfig::Std;
                    stderr = StdioConfig::Std;
                }
                _ => {
                    args.push(arg);
                }
            }
        }

        commands.push(CommandArgs {
            args: args.clone(),
            stdin: if matches!(stdin, StdioConfig::Std) && prev_piped {
                StdioConfig::Piped
            } else {
                stdin
            },
            stdout,
            stderr,
        });

        Self { commands }
    }
}
