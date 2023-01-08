//! built-in functions

use std::fmt::Debug;

use crate::environment::Environment;

use super::{callable::Callable, RuntimeError, Value};

#[derive(Clone)]
pub(crate) struct Builtin {
    pub(crate) name: String,
    pub(crate) params: Vec<Value>,
    pub(crate) fun: fn(&mut Environment, Vec<Value>) -> Value,
}

impl Callable for Builtin {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        env: &mut Environment,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok((self.fun)(env, arguments))
    }
}

impl PartialEq for Builtin {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin fn {}>", self.name)
    }
}
