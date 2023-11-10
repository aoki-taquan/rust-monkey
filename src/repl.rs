use crate::lexer::Lexer;
use crate::token::Token;
use std::io::{self, Write};
pub enum ReplMode {
    Lexre,
}

pub fn start(reple_mode:ReplMode){
    // ループ
    loop {
        // プロンプトを表示
        print!(">> ");
        // バッファをフラッシュ
        io::stdout().flush().unwrap();
        // 入力を受け取る
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match reple_mode {
            ReplMode::Lexre => {
                print_token(&input);
            }
        }

    }
}

fn print_token(input: &String)  {
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            return;
        }
    }
}
