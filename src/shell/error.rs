#[allow(unused)]
pub struct ShellError {
    pub status: i32,
    pub message: String,
}

impl ShellError {
    fn new() -> Self {
        Self {
            status: -1,
            message: "".to_string(),
        }
    }

    #[allow(unused)]
    fn status(mut self, status: i32) -> Self {
        self.status = status;
        self
    }

    fn message(mut self, message: &str) -> Self {
        self.message = String::from(message);
        self
    }
}

impl From<std::io::Error> for ShellError {
    fn from(err: std::io::Error) -> Self {
        ShellError::new().message(&format!("{}", err))
    }
}
