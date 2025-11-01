pub struct Args {
    pub args: Vec<String>,
}

impl Args {
    pub fn new(input: &str) -> Self {
        let mut args = Vec::<String>::new();

        let mut opening_quote = None;
        let mut curr_arg = String::new();

        for character in input.trim().chars() {
            if character == '\'' || character == '"' {
                if opening_quote.is_some_and(|c| c == character) {
                    opening_quote = None;
                    continue;
                } else if opening_quote.is_none() {
                    opening_quote = Some(character);
                    continue;
                }
            }

            if character.is_whitespace() && opening_quote.is_none() {
                let arg = curr_arg.trim();
                if !arg.is_empty() {
                    args.push(arg.to_string());
                }
                curr_arg.clear();
                opening_quote = None;
            } else {
                curr_arg.push(character);
            }
        }

        args.push(curr_arg);

        Self { args }
    }

    pub fn command(&self) -> &str {
        &self.args[0]
    }
}
