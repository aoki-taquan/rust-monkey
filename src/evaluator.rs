use crate::{
    ast,
    object::{
        environment::Environment,
        {EvalError, Object},
    },
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn eval(node: ast::Node, env: &mut Environment) -> Result<Rc<Object>, EvalError> {
    match node {
        ast::Node::Program(program) => eval_block_statemt(program, env),
        // 使ってないようなのでコメントアウト
        // ast::Node::Statement(statement) => eval_statement(statement, env),
        // ast::Node::Expression(expression) => eval_expression(expression, env),
    }
}

fn eval_block_statemt(
    program: ast::Program,
    env: &mut Environment,
) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);
    for hoge in program {
        result = eval_statement(hoge, env)?;

        if let Object::ReturnValue(val) = &*result {
            return Ok(val.clone());
        }
    }
    Ok(result)
}

fn eval_statement(
    statement: ast::Statement,
    env: &mut Environment,
) -> Result<Rc<Object>, EvalError> {
    match statement {
        ast::Statement::LetStatement { name, value } => {
            let value = eval_expression(value, env)?;
            env.set(&name, value.clone());
            Ok(Rc::new(Object::Null))
        }
        ast::Statement::Return { return_value } => {
            let return_value = eval_expression(return_value, env)?;
            Ok(Rc::new(Object::ReturnValue(return_value)))
        }
        ast::Statement::Expression { expression } => eval_expression(expression, env),
    }
}

fn eval_expression(
    expression: ast::Expression,
    env: &mut Environment,
) -> Result<Rc<Object>, EvalError> {
    match expression {
        ast::Expression::Identifier(identifier) => eval_identifier(identifier, env),
        ast::Expression::Boolean(b) => Ok(Rc::new(Object::Boolean(b))),
        ast::Expression::IntegerLiteral(i) => Ok(Rc::new(Object::Integer(i))),
        ast::Expression::PrefixExpression { operator, right } => {
            let right = eval_expression(*right, env)?;
            eval_prefix_expression(operator, right)
        }
        ast::Expression::InfixExpression {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(*left, env)?;
            let right = eval_expression(*right, env)?;
            eval_infix_expression(operator, left, right)
        }
        ast::Expression::IfExpression {
            condition,
            consequence,
            alternative,
        } => eval_if_expression(*condition, consequence, alternative, env),
        ast::Expression::FunctionLiteral { parameters, body } => Ok(Rc::new(Object::Function {
            parameters,
            body,
            env: env.clone(),
        })),
        ast::Expression::CallExpression {
            function,
            arguments,
        } => {
            let function = eval_expression(*function, env)?;
            let arguments = eval_expressions(arguments, env)?;
            apply_function(function, arguments)
        }
        ast::Expression::StringLiteral(s) => Ok(Rc::new(Object::String(s))),
        ast::Expression::ArrayLiteral(elements) => {
            let elements = eval_expressions(elements, env)?;
            Ok(Rc::new(Object::Array(elements)))
        }
        ast::Expression::IndexExpression { left, index } => {
            let left = eval_expression(*left, env)?;
            let index = eval_expression(*index, env)?;
            eval_index_expression(left, index)
        }
        ast::Expression::HashLiteral(pairs) => eval_hash_literal(pairs, env),
    }
}

fn eval_prefix_expression(
    operator: ast::PrefixOperator,
    right: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
    match operator {
        ast::PrefixOperator::Bang => match &*right {
            Object::Boolean(b) => Ok(Rc::new(Object::Boolean(!b))),
            Object::Null => Ok(Rc::new(Object::Boolean(true))),
            _ => Ok(Rc::new(Object::Boolean(false))),
        },
        ast::PrefixOperator::Minus => match &*right {
            Object::Integer(i) => Ok(Rc::new(Object::Integer(-i))),
            _ => Err(format!("unknown operator: -{}", right)),
        },
    }
}

fn eval_infix_expression(
    operator: ast::InfixOperator,
    left: Rc<Object>,
    right: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
    match (&*left, &*right) {
        (Object::Integer(l), Object::Integer(r)) => match operator {
            ast::InfixOperator::Plus => Ok(Rc::new(Object::Integer(l + r))),
            ast::InfixOperator::Minus => Ok(Rc::new(Object::Integer(l - r))),
            ast::InfixOperator::Asterisk => Ok(Rc::new(Object::Integer(l * r))),
            ast::InfixOperator::Slash => Ok(Rc::new(Object::Integer(l / r))),
            ast::InfixOperator::Lt => Ok(Rc::new(Object::Boolean(l < r))),
            ast::InfixOperator::Gt => Ok(Rc::new(Object::Boolean(l > r))),
            ast::InfixOperator::Eq => Ok(Rc::new(Object::Boolean(l == r))),
            ast::InfixOperator::NotEq => Ok(Rc::new(Object::Boolean(l != r))),
        },
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            ast::InfixOperator::Eq => Ok(Rc::new(Object::Boolean(l == r))),
            ast::InfixOperator::NotEq => Ok(Rc::new(Object::Boolean(l != r))),
            _ => Err(format!("unknown operator: {} {} {}", left, operator, right)),
        },
        (Object::String(l), Object::String(r)) => match operator {
            ast::InfixOperator::Plus => Ok(Rc::new(Object::String(Rc::new(format!("{}{}", l, r))))),
            ast::InfixOperator::Eq => Ok(Rc::new(Object::Boolean(l == r))),
            ast::InfixOperator::NotEq => Ok(Rc::new(Object::Boolean(l != r))),
            _ => Err(format!("unknown operator: {} {} {}", left, operator, right)),
        },
        _ => Err(format!("type mismatch: {} {} {}", left, operator, right)),
    }
}

fn eval_if_expression(
    condition: ast::Expression,
    consequence: ast::BlockStatement,
    alternative: Option<ast::BlockStatement>,
    env: &mut Environment,
) -> Result<Rc<Object>, EvalError> {
    let condition = eval_expression(condition, env)?;
    if is_truthy(condition) {
        eval_block_statemt(consequence, env)
    } else {
        match alternative {
            Some(alt) => eval_block_statemt(alt, env),
            None => Ok(Rc::new(Object::Null)),
        }
    }
}

fn is_truthy(obj: Rc<Object>) -> bool {
    match &*obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn eval_expressions(
    expressions: Vec<ast::Expression>,
    env: &mut Environment,
) -> Result<Vec<Rc<Object>>, EvalError> {
    let mut result = Vec::new();
    for expression in expressions {
        let evaluated = eval_expression(expression, env)?;
        result.push(evaluated);
    }
    Ok(result)
}

fn apply_function(
    function: Rc<Object>,
    arguments: Vec<Rc<Object>>,
) -> Result<Rc<Object>, EvalError> {
    match &*function {
        Object::Function {
            parameters,
            body,
            env,
        } => {
            let mut extended_env = new_enclosed_environment(env.clone());
            for (i, param) in parameters.iter().enumerate() {
                extended_env.set(&param, arguments[i].clone());
            }
            let evaluated = eval_block_statemt(body.clone(), &mut extended_env)?;
            if let Object::ReturnValue(val) = &*evaluated {
                Ok(val.clone())
            } else {
                Ok(evaluated)
            }
        }
        // todo:後でサポート予定
        // Object::BuiltinFunction(f) => Ok(f(arguments)),
        _ => Err(format!("not a function: {}", function)),
    }
}

fn eval_index_expression(left: Rc<Object>, index: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match (&*left, &*index) {
        (Object::Array(elements), Object::Integer(i)) => {
            if *i < 0 || *i >= elements.len() as i64 {
                return Ok(Rc::new(Object::Null));
            }
            Ok(elements[*i as usize].clone())
        }
        (Object::Hash(pairs), _) => {
            if let Some(value) = pairs.get(&index) {
                Ok(value.clone())
            } else {
                Ok(Rc::new(Object::Null))
            }
        }
        _ => Err(format!("index operator not supported: {}", left)),
    }
}

fn eval_hash_literal(
    pairs: Vec<ast::HashPair>,
    env: &mut Environment,
) -> Result<Rc<Object>, EvalError> {
    let mut result = HashMap::new();
    for pair in pairs {
        let key = eval_expression(pair.key, env)?;
        let value = eval_expression(pair.value, env)?;
        result.insert(key, value);
    }
    Ok(Rc::new(Object::Hash(result)))
}

fn eval_identifier(name: Rc<String>, env: &mut Environment) -> Result<Rc<Object>, EvalError> {
    match env.get(&name) {
        Some(o) => Ok(o),
        None => Err(format!("identifier not found: {}", name)),
    }
}

pub fn new_enclosed_environment(outer: Environment) -> Environment {
    let mut env = new_environment();
    env.outer = Some(Rc::new(RefCell::new(outer)));
    env
}

pub fn new_environment() -> Environment {
    Environment {
        store: HashMap::new(),
        outer: None,
    }
}
