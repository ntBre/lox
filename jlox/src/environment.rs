use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::{
    interpreter::{value::Value, RuntimeError},
    token::Token,
};

type StackVal = HashMap<String, Rc<RefCell<Value>>>;

/// NOTE instead of representing an Environment as a HashMap with an optional
/// enclosing HashMap, which led to disastrous lifetime issues, we model the
/// environment as a stack of HashMaps with a pointer (index) to the current
/// entry. Traversing the list of parents becomes decrementing current and
/// recursing
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Environment {
    pub(crate) stack: Vec<StackVal>,
}

impl Index<usize> for Environment {
    type Output = StackVal;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stack[index]
    }
}

impl IndexMut<usize> for Environment {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stack[index]
    }
}

type EnvResult = Result<Rc<RefCell<Value>>, RuntimeError>;

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
        self.stack[i].insert(name, Rc::new(RefCell::new(value)));
    }

    pub(crate) fn get(&mut self, name: Token) -> EnvResult {
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

    pub(crate) fn get_at(&mut self, distance: usize, name: Token) -> EnvResult {
        match self.stack[self.ancestor(distance)].get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(RuntimeError::new(
                format!("Undefined variable '{}'.", name.lexeme),
                name,
            )),
        }
    }

    pub(crate) fn assign(
        &mut self,
        name: Token,
        value: Value,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        for i in (0..self.stack.len()).rev() {
            if self.stack[i].contains_key(&name.lexeme) {
                let mut b =
                    self.stack[i].get(&name.lexeme).unwrap().borrow_mut();
                *b = value;
                return Ok(self.stack[i].get(&name.lexeme).unwrap().clone());
            }
        }
        Err(RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme),
            name,
        ))
    }

    pub(crate) fn assign_at(
        &mut self,
        distance: usize,
        name: Token,
        value: Value,
    ) -> EnvResult {
        // looks a bit suspicious unwrapping, but I guess we know the variable
        // has been resolved from the resolver
        let i = self.ancestor(distance);
        let mut b = self.stack[i].get(&name.lexeme).unwrap().borrow_mut();
        *b = value;
        return Ok(self.stack[i].get(&name.lexeme).unwrap().clone());
    }

    /// if len is 9 and distance is 0, need to return 8, the last valid index
    fn ancestor(&self, distance: usize) -> usize {
        self.stack.len() - distance - 1
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
