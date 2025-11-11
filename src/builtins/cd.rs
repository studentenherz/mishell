use std::env;
use std::path::Path;

use crate::args::CommandArgs;
use crate::commands::*;
use crate::shell::Shell;

pub struct Cd;

impl Cd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for Cd {
    fn eval(
        &self,
        _shell_ctx: &mut Shell,
        args: CommandArgs,
        _stdin: Box<dyn Read>,
        _stdout: Box<dyn Write>,
        mut stderr: Box<dyn Write>,
    ) -> CommandReturnType {
        let arg1 = args.args.iter().nth(1).map_or("~", |x| x.as_str());
        let home_dir = env::home_dir().map_or("~".to_string(), |x| x.display().to_string());

        let target_dir = &arg1.replace("~", &home_dir);
        let target_dir_path = Path::new(target_dir);

        if matches!(target_dir_path.try_exists(), Ok(true)) && target_dir_path.is_dir() {
            if env::set_current_dir(target_dir_path).is_ok() {
                return CommandReturnType {};
            }
        }

        let output = format!("cd: {}: No such file or directory\n", arg1);
        stderr.write_all(output.as_bytes()).unwrap();
        stderr.flush().unwrap();

        CommandReturnType {}
    }
}
