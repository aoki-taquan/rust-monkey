#[cfg(test)]
mod tests {
    // use super::*;
    use crate::lexer::Lexer;
    use crate::token::Token::{self};
    use std::rc::Rc;

    fn token_test(input: &str, expected_tokens: Vec<Token>) {
        let mut lexer = Lexer::new(&input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_lex_single_token() {
        let input = "let five = 5;";
        let expected_output = vec![
            Token::Let,
            Token::Ident(Rc::new("five".to_string())),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::EOF,
        ];
        token_test(input, expected_output);
    }

    #[test]
    fn test_lex_multiple_tokens() {
        let input = "let ten = 10;
                     let add = fn(x, y) {
                         x + y;
                     };
                     let result = add(five, ten);";
        let expected_output = vec![
            Token::Let,
            Token::Ident(Rc::new("ten".to_string())),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(Rc::new("add".to_string())),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(Rc::new("x".to_string())),
            Token::Comma,
            Token::Ident(Rc::new("y".to_string())),
            Token::RParen,
            Token::LBrace,
            Token::Ident(Rc::new("x".to_string())),
            Token::Plus,
            Token::Ident(Rc::new("y".to_string())),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(Rc::new("result".to_string())),
            Token::Assign,
            Token::Ident(Rc::new("add".to_string())),
            Token::LParen,
            Token::Ident(Rc::new("five".to_string())),
            Token::Comma,
            Token::Ident(Rc::new("ten".to_string())),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ];
        token_test(input, expected_output);
    }

    #[test]
    fn test_lex_tokens() {
        let input = r#"let five = 5;
        let ten=10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
        	return true;
        } else {
        	return false;
        }

        10 == 10;
        10 != 9;
        "foobar"
        "foo bar"
        [1, 2];
        {"foo": "bar"}"#;

        let expected_output = vec![
            Token::Let,
            Token::Ident(Rc::new("five".to_string())),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(Rc::new("ten".to_string())),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(Rc::new("add".to_string())),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(Rc::new("x".to_string())),
            Token::Comma,
            Token::Ident(Rc::new("y".to_string())),
            Token::RParen,
            Token::LBrace,
            Token::Ident(Rc::new("x".to_string())),
            Token::Plus,
            Token::Ident(Rc::new("y".to_string())),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(Rc::new("result".to_string())),
            Token::Assign,
            Token::Ident(Rc::new("add".to_string())),
            Token::LParen,
            Token::Ident(Rc::new("five".to_string())),
            Token::Comma,
            Token::Ident(Rc::new("ten".to_string())),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::String(Rc::new("foobar".to_string())),
            Token::String(Rc::new("foo bar".to_string())),
            Token::LBracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::RBracket,
            Token::Semicolon,
            Token::LBrace,
            Token::String(Rc::new("foo".to_string())),
            Token::Colon,
            Token::String(Rc::new("bar".to_string())),
            Token::RBrace,
            Token::EOF,
        ];
        token_test(input, expected_output);
    }
}
