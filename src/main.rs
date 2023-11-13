mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
mod ast;
use repl::ReplMode;

fn main() {
    // usernameを表示
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    let repl_mode = ReplMode::Parser;
    repl::start(repl_mode);
}
