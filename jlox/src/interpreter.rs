use std::fmt::Display;

use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Value {
    /// [Value::Nil] and false are falsey, everything else is truthy
    fn is_truthy(&self) -> bool {
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
        }
    }
}

macro_rules! with_strings {
    ($op:ident, $($left:ident => $a:ident$(,)*)*) => {
	$(
	    let Value::String($a) = $left else {
		return Err(RuntimeError::new(
		    format!("expected String, got {:?}", $left),
		    $op,
		))
	    };
	)*
    };
}

macro_rules! with_numbers {
    ($op:ident, $($left:ident => $a:ident$(,)*)*) => {
	$(
	    let Value::Number($a) = $left else {
		return Err(RuntimeError::new(
		    format!("expected Number, got {:?}", $left),
		    $op,
		))
	    };
	)*
    };
}

pub(crate) struct RuntimeError {
    message: String,
    token: Token,
}

impl RuntimeError {
    pub(crate) fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }

    pub(crate) fn message(&self) -> &str {
        &(self.message)
    }

    pub(crate) fn line(&self) -> &Token {
        &(self.token)
    }
}

impl Stmt {
    pub(crate) fn execute(self) -> Result<Value, RuntimeError> {
        match self {
            Stmt::Expression(e) => e.evaluate(),
            Stmt::Print(e) => {
                let value = e.evaluate()?;
                println!("{}", value);
                Ok(value)
            }
        }
    }
}

impl Expr {
    /// consume the expression in `self` and evaluate it to a [Value]
    pub(crate) fn evaluate(self) -> Result<Value, RuntimeError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match operator.typ {
                    TokenType::Minus => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Number(a - b))
                    }
                    TokenType::Slash => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Number(a / b))
                    }
                    TokenType::Star => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Number(a * b))
                    }
                    TokenType::Plus => {
                        if matches!(left, Value::Number(_))
                            && matches!(right, Value::Number(_))
                        {
                            with_numbers!(operator, left => a, right => b);
                            Ok(Value::Number(a + b))
                        } else {
                            with_strings!(operator, left => a, right => b);
                            Ok(Value::String(a + &b))
                        }
                    }
                    // NOTE comparisons are only supported for numbers, but I
                    // could trivially support them for any Value by deriving
                    // PartialOrd
                    TokenType::Greater => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Boolean(a > b))
                    }
                    TokenType::GreaterEqual => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Boolean(a >= b))
                    }
                    TokenType::Less => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Boolean(a < b))
                    }
                    TokenType::LessEqual => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Value::Boolean(a <= b))
                    }
                    TokenType::BangEqual => {
                        Ok(Value::Boolean(!(left == right)))
                    }
                    TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Literal(l) => match l {
                Literal::String(s) => Ok(Value::String(s)),
                Literal::Number(n) => Ok(Value::Number(n)),
                Literal::True => Ok(Value::Boolean(true)),
                Literal::False => Ok(Value::Boolean(false)),
                Literal::Null => Ok(Value::Nil),
            },
            Expr::Unary { operator, right } => {
                let right = right.evaluate()?;
                match operator.typ {
                    TokenType::Minus => {
                        with_numbers!(operator, right => n);
                        Ok(Value::Number(-n))
                    }
                    TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
                    _ => unreachable!(),
                }
            }
            Expr::Null => unreachable!(),
        }
    }
}
