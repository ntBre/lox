use std::fmt::Display;

use crate::{
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
};

use self::{builtin::Builtin, callable::Callable, function::Function};

pub(crate) mod builtin;
mod callable;
mod function;

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
            Value::Function(fun) => write!(f, "{fun}"),
            Value::Builtin(b) => write!(f, "{b:?}"),
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

pub(crate) enum RuntimeError {
    Error { message: String, token: Token },
    Return(Value),
}

impl RuntimeError {
    pub(crate) fn new(message: String, token: Token) -> Self {
        Self::Error { message, token }
    }

    pub(crate) fn message(&self) -> &str {
        match self {
            RuntimeError::Error { message, token: _ } => message,
            RuntimeError::Return(_) => unreachable!(),
        }
    }

    pub(crate) fn line(&self) -> &Token {
        match self {
            RuntimeError::Error { message: _, token } => token,
            RuntimeError::Return(_) => unreachable!(),
        }
    }
}

impl Stmt {
    pub(crate) fn execute(
        self,
        env: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        match self {
            Stmt::Expression { expression: e } => e.evaluate(env),
            Stmt::Print { expression: e } => {
                let value = e.evaluate(env)?;
                println!("{value}");
                Ok(value)
            }
            Stmt::Var { name, initializer } => {
                let value = if !initializer.is_null() {
                    initializer.evaluate(env)?
                } else {
                    Value::Nil
                };
                env.define(name.lexeme, value);
                Ok(Value::Nil)
            }
            Stmt::Block { statements } => {
                env.push();
                for statement in statements {
                    if let e @ Err(_) = statement.execute(env) {
                        // have to reset the stack before returning in case of
                        // error, so we can't just use ?
                        env.pop();
                        return e;
                    }
                }
                env.pop();
                Ok(Value::Nil)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(env)?.is_truthy() {
                    Ok(then_branch.execute(env)?)
                } else if !else_branch.is_null() {
                    Ok(else_branch.execute(env)?)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::Null => todo!(),
            Stmt::While { condition, body } => {
                // these clones feel a bit weird. letting execute and evaluate
                // take &self seems okay as an alternative, but then I have to
                // clone the strings and numbers instead.
                while condition.clone().evaluate(env)?.is_truthy() {
                    body.clone().execute(env)?;
                }
                Ok(Value::Nil)
            }
            Stmt::Function { name, params, body } => {
                let function = Function::new(
                    Stmt::Function {
                        name: name.clone(),
                        params,
                        body,
                    },
                    env.clone(),
                );
                env.define(name.lexeme, Value::Function(function));
                Ok(Value::Nil)
            }
            Stmt::Return { keyword: _, value } => {
                let ret = if !value.is_null() {
                    value.evaluate(env)?
                } else {
                    Value::Nil
                };
                Err(RuntimeError::Return(ret))
            }
        }
    }
}

impl Expr {
    /// consume the expression in `self` and evaluate it to a [Value]
    pub(crate) fn evaluate(
        self,
        env: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env)?;
                let right = right.evaluate(env)?;

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
            Expr::Grouping { expression } => expression.evaluate(env),
            Expr::Literal(l) => match l {
                Literal::String(s) => Ok(Value::String(s)),
                Literal::Number(n) => Ok(Value::Number(n)),
                Literal::True => Ok(Value::Boolean(true)),
                Literal::False => Ok(Value::Boolean(false)),
                Literal::Null => Ok(Value::Nil),
            },
            Expr::Unary { operator, right } => {
                let right = right.evaluate(env)?;
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
            Expr::Variable { name } => env.get(name),
            Expr::Assign { name, value } => {
                let value = value.evaluate(env)?;
                // NOTE this is a little different from the Java version because
                // I've made `assign` clone and return the value again instead
                // of cloning here and then returning value. I don't think it
                // will make much difference overall, and it means I can return
                // Result<Value, RuntimeError> from assign instead of Result<(),
                // RuntimeError> and process that here
                env.assign(name, value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env)?;
                if operator.typ.is_or() && left.is_truthy() {
                    return Ok(left);
                }
                if !left.is_truthy() {
                    return Ok(left);
                }
                right.evaluate(env)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let function = callee.evaluate(env)?;

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(arg.evaluate(env)?);
                }

                match function {
                    Value::Function(f) => finish_callable(f, args, paren, env),
                    Value::Builtin(b) => finish_callable(b, args, paren, env),
                    _ => Err(RuntimeError::new(
                        "Can only call functions and classes.".to_owned(),
                        paren,
                    )),
                }
            }
        }
    }
}

fn finish_callable(
    mut fun: impl Callable,
    args: Vec<Value>,
    paren: Token,
    env: &mut Environment,
) -> Result<Value, RuntimeError> {
    if args.len() != fun.arity() {
        return Err(RuntimeError::new(
            format!(
                "Expected {} arguments but got {}.",
                fun.arity(),
                args.len()
            ),
            paren,
        ));
    }
    fun.call(env, args)
}
