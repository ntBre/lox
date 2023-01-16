use std::{cell::RefCell, rc::Rc};

use super::{RuntimeError, Value, Interpreter};
use crate::environment::Environment;

pub(crate) trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &mut self,
        int: &mut Interpreter,
        arguments: Vec<Rc<RefCell<Value>>>,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError>;
}
