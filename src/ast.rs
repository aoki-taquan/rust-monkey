use core::fmt;
use std::rc::Rc;

pub type Program = Vec<Statement>;

pub enum Node {
    Program(Program),
    //　使ってないようなのでコメントアウト
    // Statement(Statement),
    // Expression(Expression),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Statement {
    LetStatement {
        name: Identifier,
        value: Expression,
    },
    Return {
        return_value: Expression,
    },
    Expression {
        expression: Expression,
    },
    // 使ってないようなのでコメントアウト
    // Block {
    //     statements: Vec<Statement>,
    // },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement { name, value } => write!(f, "let {} = {};", name, value),
            Statement::Return { return_value } => write!(f, "return {};", return_value),
            Statement::Expression { expression } => write!(f, "{}", expression),
            // 使ってないようなのでコメントアウト
            // Statement::Block { statements } => {
            //     write!(f, "{}", fmt_vec(statements))
            // }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Boolean(bool),
    IntegerLiteral(i64),
    PrefixExpression {
        operator: PrefixOperator,
        right: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
        FunctionLiteral {
        parameters: Vec<Identifier>,
        body: BlockStatement,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StringLiteral(Rc<String>),
    ArrayLiteral(Vec<Expression>),
    IndexExpression {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    HashLiteral(Vec<HashPair>),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct HashPair {
    pub key: Expression,
    pub value: Expression,
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
                for map in hash {
                    result = format!("{}{}: {}, ", result, map.key, map.value);
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum InfixOperator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
}

impl fmt::Display for InfixOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfixOperator::Plus => write!(f, "+"),
            InfixOperator::Minus => write!(f, "-"),
            InfixOperator::Asterisk => write!(f, "*"),
            InfixOperator::Slash => write!(f, "/"),
            InfixOperator::Lt => write!(f, "<"),
            InfixOperator::Gt => write!(f, ">"),
            InfixOperator::Eq => write!(f, "=="),
            InfixOperator::NotEq => write!(f, "!="),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum PrefixOperator {
    Bang,
    Minus,
}

impl fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrefixOperator::Bang => write!(f, "!"),
            PrefixOperator::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
}

pub type Identifier = Rc<String>;

pub type BlockStatement = Vec<Statement>;
