mod args;
mod builtins;
mod commands;
mod eval;
mod locate;
mod shell;
mod trie;

use shell::Shell;

fn main() -> std::io::Result<()> {
    let mut shell = Shell::new();
    shell.run()
}
