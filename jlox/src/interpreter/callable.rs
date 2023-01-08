use super::{RuntimeError, Value};
use crate::environment::Environment;

pub(crate) trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &self,
        env: &mut Environment,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError>;
}
