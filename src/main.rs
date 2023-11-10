mod lexer;
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
    let repl_mode = ReplMode::Lexre;
    repl::start(repl_mode);
}
