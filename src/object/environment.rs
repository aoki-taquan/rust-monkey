use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Environment {
    pub store: HashMap<String, Rc<Object>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("{{");
        for (i, (k, v)) in self.store.iter().enumerate() {
            if i != 0 {
                s.push_str(", ");
            }
            s.push_str(&format!("{}: {}", k, v));
        }
        s.push_str("}}");
        write!(f, "{}", s)
    }
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(o) => Some(o.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str, val: Rc<Object>) -> Option<Rc<Object>> {
        self.store.insert(name.to_string(), val.clone())
    }
}
