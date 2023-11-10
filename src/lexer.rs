use crate::token::Token;
mod test;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &String) -> Lexer {
        let input: Vec<char> = input.chars().collect();
        match input.get(0) {
            Some(&ch) => Lexer {
                input: input,
                position: 0,
                read_position: 1,
                ch: ch,
            },
            None => Lexer {
                input: input,
                position: 0,
                read_position: 0,
                ch: '\0',
            },
        }
    }

    pub fn next_token(&mut self) -> Token {
        Self::sukip_white_spaces(self);

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '"' => Token::String(Self::read_string(self)),
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '\0' => Token::EOF,
            _ => {
                // TODO:hoge9もできるようにする
                if Self::is_letter(self.ch) {
                    let literal = Self::read_identifier(self);
                    return Token::lookup_ident(literal);
                } else if Self::is_digit(self.ch) {
                    return Token::Int(Self::read_number(self));
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn sukip_white_spaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        match self.input.get(self.read_position) {
            Some(&ch) => ch,
            None => '\0',
        }
    }

    //未確認
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn read_number(&mut self) -> i64 {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        self.input[position..self.position].iter().collect()
    }

    // TODO:UTF-8に対応してもいいんじゃね
    fn is_letter(ch: char) -> bool {
        ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
    }
}
