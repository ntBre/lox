use std::{fmt::Display, ops::Index};

#[derive(Default, Clone, Copy, Debug)]
pub enum Value {
    Bool(bool),
    #[default]
    Nil,
    Number(f64),
}

impl Value {
    pub(crate) fn boolean(v: bool) -> Self {
        Self::Bool(v)
    }

    pub(crate) fn nil() -> Self {
        Self::Nil
    }

    pub(crate) fn number(v: f64) -> Self {
        Self::Number(v)
    }

    pub(crate) fn is_falsey(&self) -> bool {
        self.is_nil() || (self.is_bool() && !self.as_bool().unwrap())
    }

    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Bool`].
    ///
    /// [`Bool`]: Value::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `true` if the value is [`Nil`].
    ///
    /// [`Nil`]: Value::Nil
    #[must_use]
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    /// Returns `true` if the value is [`Number`].
    ///
    /// [`Number`]: Value::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    pub fn as_number(&self) -> Option<&f64> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            _ => false,
        }
    }
}

// corresponds to printValue
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{n}"),
        }
    }
}

// this is probably not needed, but we'll see. keeping consistent with C
// version for now. alternative would be constants: Vec<Value> directly on
// Chunk
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }
}

impl Default for ValueArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for ValueArray {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
