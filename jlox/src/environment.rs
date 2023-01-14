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
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Environment {
    stack: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    /// add a new frame to self and adjust the stack pointer to point to it
    pub(crate) fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    /// pop a stack frame from self and adjust the stack pointer to point to the
    /// end. panics if the stack is empty
    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn define(&mut self, name: String, value: Value) {
        let i = self.stack.len() - 1;
        self.stack[i].insert(name, value);
    }

    pub(crate) fn get(&mut self, name: Token) -> Result<Value, RuntimeError> {
        for i in (0..self.stack.len()).rev() {
            if let Some(v) = self.stack[i].get(&name.lexeme) {
                // this is sad, but I have to clone. I guess that's what java
                // does?
                return Ok(v.clone());
            }
        }
        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme),
            name,
        ))
    }

    pub(crate) fn assign(
        &mut self,
        name: Token,
        value: Value,
    ) -> Result<Value, RuntimeError> {
        for i in (0..self.stack.len()).rev() {
            if self.stack[i].contains_key(&name.lexeme) {
                self.define(name.lexeme, value.clone());
                return Ok(value);
            }
        }
        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme),
            name,
        ))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
