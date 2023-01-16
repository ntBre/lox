use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
    Lox,
};

use self::{
    builtin::Builtin, callable::Callable, function::Function, value::Value,
};

pub(crate) mod builtin;
mod callable;
mod function;
pub(crate) mod value;

pub(crate) struct Interpreter<'a> {
    pub(crate) lox: &'a mut Lox,
    globals: Environment,

    /// index of the current environment in globals
    environment: usize,

    locals: HashMap<Expr, usize>,
}

fn clock(
    _: &mut Environment,
    _: Vec<Rc<RefCell<Value>>>,
) -> Rc<RefCell<Value>> {
    Rc::new(RefCell::new(Value::Number(
        std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .unwrap()
            .as_millis() as f64
            / 1000.0,
    )))
}

impl<'a> Interpreter<'a> {
    pub(crate) fn new(lox: &'a mut Lox) -> Self {
        let mut globals = Environment::new();
        globals.define(
            "clock".to_owned(),
            Value::Builtin(Builtin {
                params: Vec::new(),
                fun: clock,
            }),
        );
        Self {
            lox,
            globals,
            environment: 0,
            locals: HashMap::new(),
        }
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                self.lox.runtime_error(e);
            }
        }
    }

    pub(crate) fn resolve(&mut self, expr: Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }
}

macro_rules! with_strings {
    ($op:ident, $($left:ident => $a:ident$(,)*)*) => {
	$(
	    let Value::String($a) = $left.borrow().clone() else {
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
	    let Value::Number($a) = *$left.borrow() else {
		return Err(RuntimeError::new(
		    format!("Operands must be numbers."),
		    $op,
		))
	    };
	)*
    };
}

#[derive(Debug)]
pub(crate) enum RuntimeError {
    Error { message: String, token: Token },
    Return(Rc<RefCell<Value>>),
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

    pub(crate) fn line(&self) -> usize {
        match self {
            RuntimeError::Error { message: _, token } => token.line,
            RuntimeError::Return(_) => unreachable!(),
        }
    }
}

impl<'a> Interpreter<'a> {
    pub(crate) fn execute(
        &mut self,
        stmt: Stmt,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        match stmt {
            Stmt::Expression { expression: e } => self.evaluate(e),
            Stmt::Print { expression: e } => {
                let value = self.evaluate(e)?;
                println!("{}", value.borrow());
                Ok(value)
            }
            Stmt::Var { name, initializer } => {
                let value = if !initializer.is_null() {
                    self.evaluate(initializer)?
                } else {
                    Rc::new(RefCell::new(Value::Nil))
                };
                self.globals.define(name.lexeme, value.borrow().clone());
                Ok(Rc::new(RefCell::new(Value::Nil)))
            }
            Stmt::Block { statements } => {
                self.globals.push();
                for statement in statements {
                    if let e @ Err(_) = self.execute(statement) {
                        // have to reset the stack before returning in case of
                        // error, so we can't just use ?
                        self.globals.pop();
                        return e;
                    }
                }
                self.globals.pop();
                Ok(Rc::new(RefCell::new(Value::Nil)))
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.borrow().is_truthy() {
                    Ok(self.execute(*then_branch)?)
                } else if !else_branch.is_null() {
                    Ok(self.execute(*else_branch)?)
                } else {
                    Ok(Rc::new(RefCell::new(Value::Nil)))
                }
            }
            Stmt::Null => todo!(),
            Stmt::While { condition, body } => {
                // these clones feel a bit weird. letting execute and evaluate
                // take &self seems okay as an alternative, but then I have to
                // clone the strings and numbers instead.
                while self.evaluate(condition.clone())?.borrow().is_truthy() {
                    self.execute(*body.clone())?;
                }
                Ok(Rc::new(RefCell::new(Value::Nil)))
            }
            Stmt::Function { name, params, body } => {
                let function = Function::new(
                    Stmt::Function {
                        name: name.clone(),
                        params,
                        body,
                    },
                    self.globals.clone(),
                );
                self.globals.define(name.lexeme, Value::Function(function));
                Ok(Rc::new(RefCell::new(Value::Nil)))
            }
            Stmt::Return { keyword: _, value } => {
                let ret = if !value.is_null() {
                    self.evaluate(value)?
                } else {
                    Rc::new(RefCell::new(Value::Nil))
                };
                Err(RuntimeError::Return(ret))
            }
        }
    }

    /// consume the expression in `self` and evaluate it to a [Value]
    pub(crate) fn evaluate(
        &mut self,
        expr: Expr,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                match operator.typ {
                    TokenType::Minus => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Number(a - b))))
                    }
                    TokenType::Slash => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Number(a / b))))
                    }
                    TokenType::Star => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Number(a * b))))
                    }
                    TokenType::Plus => {
                        if matches!(*left.borrow(), Value::Number(_))
                            && matches!(*right.borrow(), Value::Number(_))
                        {
                            with_numbers!(operator, left => a, right => b);
                            Ok(Rc::new(RefCell::new(Value::Number(a + b))))
                        } else if matches!(*left.borrow(), Value::String(_))
                            && matches!(*right.borrow(), Value::String(_))
                        {
                            with_strings!(operator, left => a, right => b);
                            Ok(Rc::new(RefCell::new(Value::String(a + &b))))
                        } else {
                            Err(RuntimeError::new(
                                "Operands must be two numbers or two strings."
                                    .to_string(),
                                operator,
                            ))
                        }
                    }
                    // NOTE comparisons are only supported for numbers, but I
                    // could trivially support them for any Value by deriving
                    // PartialOrd
                    TokenType::Greater => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Boolean(a > b))))
                    }
                    TokenType::GreaterEqual => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Boolean(a >= b))))
                    }
                    TokenType::Less => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Boolean(a < b))))
                    }
                    TokenType::LessEqual => {
                        with_numbers!(operator, left => a, right => b);
                        Ok(Rc::new(RefCell::new(Value::Boolean(a <= b))))
                    }
                    TokenType::BangEqual => Ok(Rc::new(RefCell::new(
                        Value::Boolean(!(left == right)),
                    ))),
                    TokenType::EqualEqual => {
                        Ok(Rc::new(RefCell::new(Value::Boolean(left == right))))
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Literal(l) => match l {
                Literal::String(s) => {
                    Ok(Rc::new(RefCell::new(Value::String(s))))
                }
                Literal::Number(n) => {
                    Ok(Rc::new(RefCell::new(Value::Number(n))))
                }
                Literal::True => {
                    Ok(Rc::new(RefCell::new(Value::Boolean(true))))
                }
                Literal::False => {
                    Ok(Rc::new(RefCell::new(Value::Boolean(false))))
                }
                Literal::Null => Ok(Rc::new(RefCell::new(Value::Nil))),
            },
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right)?;
                match operator.typ {
                    TokenType::Minus => {
                        let Value::Number(n) = *right.borrow() else {
			    return Err(RuntimeError::new(
				"Operand must be a number.".to_owned(),
				operator,
			    ))
			};
                        Ok(Rc::new(RefCell::new(Value::Number(-n))))
                    }
                    TokenType::Bang => Ok(Rc::new(RefCell::new(
                        Value::Boolean(!right.borrow().is_truthy()),
                    ))),
                    _ => unreachable!(),
                }
            }
            Expr::Null => unreachable!(),
            Expr::Variable { name } => self.globals.get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                // NOTE this is a little different from the Java version because
                // I've made `assign` clone and return the value again instead
                // of cloning here and then returning value. I don't think it
                // will make much difference overall, and it means I can return
                // Result<Value, RuntimeError> from assign instead of Result<(),
                // RuntimeError> and process that here
                let v = value.borrow();
                self.globals.assign(name, v.clone())
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                if operator.typ.is_or() {
                    if left.borrow().is_truthy() {
                        return Ok(left);
                    }
                } else if !left.borrow().is_truthy() {
                    return Ok(left);
                }
                self.evaluate(*right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let function = self.evaluate(*callee)?;

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                let fun = function.as_ptr();
                match unsafe { &mut *fun } {
                    Value::Function(f) => self.finish_callable(f, args, paren),
                    Value::Builtin(b) => self.finish_callable(b, args, paren),
                    _ => Err(RuntimeError::new(
                        "Can only call functions and classes.".to_owned(),
                        paren,
                    )),
                }
            }
        }
    }

    fn finish_callable(
        &mut self,
        fun: &mut impl Callable,
        args: Vec<Rc<RefCell<Value>>>,
        paren: Token,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
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
        fun.call(self, args)
    }
}
