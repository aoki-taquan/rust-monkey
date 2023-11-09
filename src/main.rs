mod token;
mod lexer; 
use token::Token;


fn main() {
    // usernameを表示
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
}
