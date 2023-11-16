use crate::evaluator;
use crate::lexer::Lexer;
use crate::object::environment::Environment;
use crate::parser::Parser;
use crate::token::Token;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[allow(dead_code)]
pub enum ReplMode {
    Lexre,
    Parser,
    Eval,
}

pub fn start(reple_mode: ReplMode) {
    let mut env = Environment {
        store: HashMap::new(),
        outer: None,
    };
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
            ReplMode::Parser => {
                print_ast(&input);
            }
            ReplMode::Eval => {
                print_eval(&input, &mut env);
            }
        }
    }
}

fn print_token(input: &String) {
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            return;
        }
    }
}

fn print_ast(input: &String) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.program();

    println!("{:?}", program);
}

fn print_eval(input: &String, env: &mut Environment) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.program();

    let result = evaluator::eval(crate::ast::Node::Program(program), env);
    match result {
        Ok(obj) => println!("{}", obj),
        Err(e) => println!("{}", e),
    }
}
