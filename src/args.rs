pub struct Args {
    pub args: Vec<String>,
}

enum QuoteState {
    Single,
    Double,
    None,
}

impl Args {
    pub fn new(input: &str) -> Self {
        let mut args = Vec::<String>::new();

        let mut state = QuoteState::None;
        let mut curr_arg = String::new();
        let mut escaped = false;

        for character in input.trim().chars() {
            match state {
                _ if escaped => {
                    curr_arg.push(character);
                    escaped = false;
                }
                QuoteState::Single => {
                    if character == '\'' {
                        state = QuoteState::None;
                    } else {
                        curr_arg.push(character);
                    }
                }
                QuoteState::Double => match character {
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

        Self { args }
    }

    pub fn command(&self) -> &str {
        &self.args[0]
    }
}
