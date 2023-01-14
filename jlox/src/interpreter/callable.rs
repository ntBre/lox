use std::{cell::RefCell, rc::Rc};

use super::{RuntimeError, Value};
use crate::environment::Environment;

pub(crate) trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &mut self,
        env: &mut Environment,
        arguments: Vec<Rc<RefCell<Value>>>,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError>;
}
