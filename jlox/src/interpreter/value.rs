use std::fmt::Display;

use super::{builtin::Builtin, function::Function};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    // these should both be something like Function(Callable), but everything
    // I've tried was a disaster with generics
    Function(Function),
    Builtin(Builtin),
}

impl Value {
    /// [Value::Nil] and false are falsey, everything else is truthy
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(fun) => write!(f, "{fun}"),
            Value::Builtin(b) => write!(f, "{b:?}"),
        }
    }
}
