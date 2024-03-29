//! built-in functions

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::environment::Environment;

use super::{callable::Callable, Interpreter, RuntimeError, Value};

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub(crate) struct Builtin {
    pub(crate) params: Vec<Value>,
    pub(crate) fun:
        fn(&mut Environment, Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>>,
}

impl Callable for Builtin {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &mut self,
        int: &mut Interpreter,
        arguments: Vec<Rc<RefCell<Value>>>,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        Ok((self.fun)(&mut int.globals, arguments))
    }
}

impl PartialEq for Builtin {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
