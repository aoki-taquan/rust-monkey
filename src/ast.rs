use core::fmt;
use std::collections::HashMap;

pub type Program = Vec<Statement>;

pub enum Statement {
    LetStatement {
        // TODO: ここは識別子のみ
        name: Identifier,
        value: Expression,
    },
    Return {
        return_value: Expression,
    },
    Expression {
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement { name, value } => write!(f, "let {} = {};", name, value),
            Statement::Return { return_value } => write!(f, "return {};", return_value),
            Statement::Expression { expression } => write!(f, "{}", expression),
            Statement::Block { statements } => {
                write!(f, "{}", fmt_vec(statements))
            }
        }
    }
}

pub enum Expression {
    // TODO:identifierだけ切り出すかどうか
    Identifier(Identifier),
    Boolean(bool),
    IntegerLiteral(i64),
    PrefixExpression {
        operator: Operater,
        right: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: Operater,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FunctionLiteral {
        parameters: Vec<Expression>,
        body: BlockStatement,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StringLiteral(String),
    ArrayLiteral(Vec<Expression>),
    IndexExpression {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    HashLiteral(HashMap<Box<Expression>, Box<Expression>>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::PrefixExpression { operator, right } => {
                write!(f, "({}{})", operator, right)
            }
            Expression::InfixExpression {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => match alternative {
                Some(a) => write!(
                    f,
                    "if {} {} else {}",
                    condition,
                    fmt_vec(consequence),
                    fmt_vec(a)
                ),
                None => write!(f, "if {} {}", condition, fmt_vec(consequence)),
            },
            Expression::FunctionLiteral { parameters, body } => {
                write!(f, "fn({}) {}", fmt_vec(parameters), fmt_vec(body))
            }
            Expression::CallExpression {
                function,
                arguments,
            } => {
                let mut result = String::new();
                for a in arguments {
                    result = format!("{} {}", result, a);
                }
                result = format!("{}({})", function, result);
                write!(f, "{}", result)
            }
            Expression::StringLiteral(s) => write!(f, "{}", s),
            Expression::ArrayLiteral(elements) => {
                let mut result = String::new();
                for e in elements {
                    result = format!("{} {}", result, e);
                }
                result = format!("[{}]", result);
                write!(f, "{}", result)
            }
            Expression::IndexExpression { left, index } => {
                write!(f, "({}[{}])", left, index)
            }
            Expression::HashLiteral(hash) => {
                let mut result = String::new();
                for (k, v) in hash {
                    result = format!("{}{}: {}, ", result, k, v);
                }
                result = format!("{{{}}}", result);
                write!(f, "{}", result)
            }
        }
    }
}

fn fmt_vec<T: fmt::Display>(vec: &Vec<T>) -> String {
    vec.into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub enum Operater {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl fmt::Display for Operater {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operater::Plus => write!(f, "+"),
            Operater::Minus => write!(f, "-"),
            Operater::Asterisk => write!(f, "*"),
            Operater::Slash => write!(f, "/"),
        }
    }
}

pub type Identifier = String;

pub type BlockStatement = Vec<Statement>;
