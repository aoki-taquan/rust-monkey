mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;
use repl::ReplMode;

fn main() {
    // usernameを表示
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    let repl_mode = ReplMode::Eval;
    repl::start(repl_mode);
}
