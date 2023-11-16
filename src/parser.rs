use crate::ast::{self, *};
use crate::lexer::Lexer;
use crate::token::Token;
use core::panic;
use std::vec;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn program(&mut self) -> ast::Program {
        let mut program = Vec::new();
        while self.cur_token != Token::EOF {
            let stmt = self.statement();
            if let Some(stmt) = stmt {
                program.push(stmt);
            }

            self.next_token();
        }
        program
    }

    fn statement(&mut self) -> Option<Statement> {
        match &self.cur_token {
            Token::Let => self.let_statement(),
            Token::Return => self.return_statement(),
            _ => self.expression_statement(),
        }
    }

    fn let_statement(&mut self) -> Option<Statement> {
        let name = match self.peek_token {
            Token::Ident(ref name) => name.clone(),
            _ => return None,
        };

        self.next_token();

        if self.peek_token != Token::Assign {
            return None;
        }

        self.next_token();
        self.next_token();

        let value = match self.expression(Precedence::Lowest) {
            Some(value) => value,
            None => return None,
        };

        // ```サンプルコードの実装
        //if p.peekTokenIs(token.SEMICOLON) {
        //		p.nextToken()
        //	}
        //````
        if self.peek_token != Token::Semicolon {
            return None;
        }

        self.next_token();

        Some(Statement::LetStatement { name, value })
    }

    fn return_statement(&mut self) -> Option<Statement> {
        if self.cur_token != Token::Return {
            panic!("cur_token is not return");
        }

        self.next_token();

        let value = match self.expression(Precedence::Lowest) {
            Some(value) => value,
            None => return None,
        };

        // ```サンプルコードの実装
        //if p.peekTokenIs(token.SEMICOLON) {
        //		p.nextToken()
        //	}
        //````
        if self.peek_token != Token::Semicolon {
            return None;
        }

        self.next_token();

        Some(Statement::Return {
            return_value: value,
        })
    }

    fn expression_statement(&mut self) -> Option<Statement> {
        let expression = match self.expression(Precedence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };

        // ```サンプルコードの実装
        //if p.peekTokenIs(token.SEMICOLON) {
        //		p.nextToken()
        //	}
        //````
        //　俺が書いたコード
        // if self.peek_token != Token::Semicolon {
        //     return None;
        // }

        // self.next_token();

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Expression { expression })
    }

    fn expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_exp = match &self.cur_token {
            Token::Ident(name) => Some(Expression::Identifier(name.clone())),
            Token::Int(i) => Some(Expression::IntegerLiteral(i.clone())),
            Token::String(s) => Some(Expression::StringLiteral(s.clone())),
            Token::Bang | Token::Minus => self.prefix_expression(),
            Token::True => Some(Expression::Boolean(true)),
            Token::False => Some(Expression::Boolean(false)),
            Token::LParen => self.grouped_expression(),
            Token::If => self.if_expression(),
            Token::Function => self.function_literal(),
            Token::LBracket => self.array_literal(),
            Token::LBrace => self.hash_literal(),
            _ => {
                panic!("cur_token is not expression");
            }
        };

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Eq
                | Token::NotEq
                | Token::Lt
                | Token::Gt => {
                    self.next_token();
                    left_exp = self.infix_expression(left_exp.expect("left_exp is None"));
                }
                Token::LParen => {
                    self.next_token();
                    left_exp = self.call_expression(left_exp.expect("left_exp is None"));
                }
                Token::LBracket => {
                    self.next_token();
                    left_exp = self.index_expression(left_exp.expect("left_exp is None"));
                }
                _ => return left_exp,
            };
        }
        left_exp
    }

    fn prefix_expression(&mut self) -> Option<Expression> {
        let operator = match self.cur_token {
            Token::Bang => PrefixOperator::Bang,
            Token::Minus => PrefixOperator::Minus,
            _ => panic!("cur_token is not prefix operator"),
        };

        self.next_token();

        let right = match self.expression(Precedence::Prefix) {
            Some(right) => right,
            None => return None,
        };

        Some(Expression::PrefixExpression {
            operator,
            right: Box::new(right),
        })
    }

    fn infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = match self.cur_token {
            Token::Plus => InfixOperator::Plus,
            Token::Minus => InfixOperator::Minus,
            Token::Slash => InfixOperator::Slash,
            Token::Asterisk => InfixOperator::Asterisk,
            Token::Eq => InfixOperator::Eq,
            Token::NotEq => InfixOperator::NotEq,
            Token::Lt => InfixOperator::Lt,
            Token::Gt => InfixOperator::Gt,
            _ => panic!("cur_token is not infix operator"),
        };
        let precedece = self.cur_precedence();

        self.next_token();

        let right = match self.expression(precedece) {
            Some(right) => right,
            None => return None,
        };

        Some(Expression::InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.expression(Precedence::Lowest);

        if self.peek_token != Token::RParen {
            return None;
        }

        self.next_token();

        return exp;
    }

    fn if_expression(&mut self) -> Option<Expression> {
        if self.cur_token != Token::If {
            panic!("cur_token is not if");
        }

        if self.peek_token != Token::LParen {
            return None;
        }

        self.next_token();
        self.next_token();

        let condition = match self.expression(Precedence::Lowest) {
            Some(condition) => condition,
            None => return None,
        };

        if self.peek_token != Token::RParen {
            return None;
        }

        self.next_token();

        if self.peek_token != Token::LBrace {
            return None;
        }

        self.next_token();

        let consequence = match self.block_statement() {
            Some(consequence) => consequence,
            None => return None,
        };

        let mut alternative = None;

        if self.peek_token == Token::Else {
            self.next_token();

            if self.peek_token != Token::LBrace {
                return None;
            }

            self.next_token();

            // retun noneをは挟むのはpaseができないとnoneを返すことにしているから
            alternative = Some(match self.block_statement() {
                Some(alternative) => alternative,
                None => return None,
            });
        }

        Some(Expression::IfExpression {
            condition: Box::new(condition),
            consequence: consequence,
            alternative: alternative,
        })
    }

    fn block_statement(&mut self) -> Option<BlockStatement> {
        let mut statements = Vec::new();

        self.next_token();

        while self.cur_token != Token::RBrace && self.cur_token != Token::EOF {
            match self.statement() {
                Some(stmt) => statements.push(stmt),
                None => return None,
            }

            self.next_token();
        }

        Some(statements)
    }

    fn function_literal(&mut self) -> Option<Expression> {
        if self.cur_token != Token::Function {
            panic!("cur_token is not function");
        }

        if self.peek_token != Token::LParen {
            return None;
        }

        self.next_token();

        let parameters = match self.function_parameters() {
            Some(parameters) => parameters,
            None => return None,
        };

        if self.peek_token != Token::LBrace {
            return None;
        }

        self.next_token();

        let body = match self.block_statement() {
            Some(body) => body,
            None => return None,
        };

        Some(Expression::FunctionLiteral { parameters, body })
    }

    fn function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token == Token::RParen {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        match self.cur_token {
            Token::Ident(ref name) => identifiers.push(name.clone()),
            _ => return None,
        }

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();

            match self.cur_token {
                Token::Ident(ref name) => identifiers.push(name.clone()),
                _ => return None,
            }
        }

        if self.peek_token != Token::RParen {
            return None;
        }

        self.next_token();

        Some(identifiers)
    }

    fn call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = match self.expression_list(Token::RParen) {
            Some(arguments) => arguments,
            None => return None,
        };

        Some(Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    fn expression_list(&mut self, end: Token) -> Option<Vec<Expression>> {
        match &end {
            Token::RBrace | Token::RParen | Token::RBracket => (),
            _ => panic!("end is not ) or ] or }}. end is {:?}", end),
        }

        let mut arguments = Vec::new();

        if self.peek_token == end {
            self.next_token();
            return Some(arguments);
        }

        self.next_token();

        match self.expression(Precedence::Lowest) {
            Some(argument) => arguments.push(argument),
            None => return None,
        }

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();

            match self.expression(Precedence::Lowest) {
                Some(argument) => arguments.push(argument),
                None => return None,
            }
        }

        //ここも
        if self.peek_token != end {
            return None;
        }

        self.next_token();

        Some(arguments)
    }

    fn array_literal(&mut self) -> Option<Expression> {
        if self.cur_token != Token::LBracket {
            panic!("cur_token is not [");
        }

        let elements = match self.expression_list(Token::RBracket) {
            Some(elements) => elements,
            None => return None,
        };

        Some(Expression::ArrayLiteral(elements))
    }

    fn index_expression(&mut self, left: Expression) -> Option<Expression> {
        if self.cur_token != Token::LBracket {
            panic!("cur_token is not [");
        }

        self.next_token();

        let index = match self.expression(Precedence::Lowest) {
            Some(index) => index,
            None => return None,
        };

        if self.peek_token != Token::RBracket {
            return None;
        }

        self.next_token();

        Some(Expression::IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        })
    }

    fn hash_literal(&mut self) -> Option<Expression> {
        if self.cur_token != Token::LBrace {
            panic!("cur_token is not {{");
        }

        let mut hash = vec![];

        while self.peek_token != Token::RBrace {
            self.next_token();
            let key = match self.expression(Precedence::Lowest) {
                Some(key) => key,
                None => return None,
            };

            if self.peek_token != Token::Colon {
                return None;
            }
            self.next_token();
            self.next_token();

            let value = match self.expression(Precedence::Lowest) {
                Some(value) => value,
                None => return None,
            };

            hash.push(HashPair { key, value });

            if self.peek_token != Token::RBrace && self.peek_token != Token::Comma {
                return None;
            }

            if self.peek_token == Token::Comma {
                self.next_token();
            }
        }

        if self.peek_token != Token::RBrace {
            return None;
        }

        self.next_token();

        Some(Expression::HashLiteral(hash))
    }

    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        Self::token_precedence(&self.peek_token)
    }

    fn cur_precedence(&self) -> Precedence {
        Self::token_precedence(&self.cur_token)
    }
}
