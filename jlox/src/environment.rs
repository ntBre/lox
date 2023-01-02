use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, Value},
    token::Token,
};

/// NOTE instead of representing an Environment as a HashMap with an optional
/// enclosing HashMap, which led to disastrous lifetime issues, we model the
/// environment as a stack of HashMaps with a pointer (index) to the current
/// entry. Traversing the list of parents becomes decrementing current and
/// recursing
pub(crate) struct Environment {
    stack: Vec<HashMap<String, Value>>,
    cur: usize,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
            cur: 0,
        }
    }

    /// add a new frame to self and adjust the stack pointer to point to it
    pub(crate) fn push(&mut self) {
        self.stack.push(HashMap::new());
        self.cur = self.stack.len() - 1;
    }

    /// pop a stack frame from self and adjust the stack pointer to point to the
    /// end. panics if the stack is empty
    pub(crate) fn pop(&mut self) {
        self.stack.pop();
        self.cur = self.stack.len() - 1;
    }

    pub(crate) fn define(&mut self, name: String, value: Value) {
        self.stack[self.cur].insert(name, value);
    }

    pub(crate) fn get(&mut self, name: Token) -> Result<Value, RuntimeError> {
        match self.stack[self.cur].get(&name.lexeme) {
            // this is sad, but I have to clone. I guess that's what java does?
            Some(v) => Ok(v.clone()),
            None => {
                if self.cur > 0 {
                    self.cur -= 1;
                    let res = self.get(name);
                    self.cur += 1;
                    res
                } else {
                    Err(RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme),
                        name,
                    ))
                }
            }
        }
    }

    pub(crate) fn assign(
        &mut self,
        name: Token,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        if self.stack[self.cur].contains_key(&name.lexeme) {
            self.define(name.lexeme, value.clone());
            Ok(value)
        } else if self.cur > 0 {
            self.cur -= 1;
            self.assign(name, value)
        } else {
            Err(RuntimeError::new(
                format!("Undefined variable '{}'.", name.lexeme),
                name,
            ))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
