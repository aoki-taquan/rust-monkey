pub mod environment;

use crate::ast::BlockStatement;
use crate::ast::Identifier;
use crate::object::environment::Environment;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// todo:後でサポート予定
// type BuiltinFunction = fn(Vec<Rc<Object>>) -> Rc<Object>;

pub type EvalError = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    String(Rc<String>),
    ReturnValue(Rc<Object>),
    Function {
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Environment,
    },
    // todo:後でサポート予定
    // BuiltinFunction(BuiltinFunction),
    Array(Vec<Rc<Object>>),
    Hash(HashMap<Rc<Object>, Rc<Object>>),
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer(i) => i.hash(state),
            Object::Boolean(b) => b.hash(state),
            Object::String(s) => s.hash(state),
            _ => panic!("unhashable type: {:?}", self),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::ReturnValue(o) => write!(f, "{}", o),
            Object::Function {
                parameters: _,
                body: _,
                env: _,
            } => write!(f, "Function"),
            // todo:後でサポート予定
            // Object::BuiltinFunction(_) => write!(f, "BuiltinFunction"),
            Object::Array(v) => {
                let mut s = String::from("[");
                for (i, o) in v.iter().enumerate() {
                    if i != 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{}", o));
                }
                s.push_str("]");
                write!(f, "{}", s)
            }
            Object::Hash(h) => {
                let mut s = String::from("{");
                for (i, (k, v)) in h.iter().enumerate() {
                    if i != 0 {
                        s.push_str(",");
                    }
                    s.push_str(&format!("{}: {}", k, v));
                }
                s.push_str("}");
                write!(f, "{}", s)
            }
        }
    }
}
