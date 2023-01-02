use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, Value},
    token::Token,
};

pub(crate) struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            // this is sad, but I have to clone. I guess that's what java does?
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
    ) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme, value.clone());
            Ok(value)
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
