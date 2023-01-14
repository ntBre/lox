use super::callable::Callable;
use super::RuntimeError;
use super::Value;
use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::token::Token;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) params: Vec<Token>,
    pub(crate) body: Vec<Stmt>,
    pub(crate) closure: Environment,
}

impl Function {
    pub(crate) fn new(declaration: Stmt, closure: Environment) -> Self {
        let Stmt::Function { name, params, body } = declaration else {
	        panic!("attempted to call non-function {declaration:?}");
	    };
        Self {
            name: name.lexeme,
            params,
            body,
            closure,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &mut self,
        _env: &mut Environment,
        arguments: Vec<Rc<RefCell<Value>>>,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        self.closure.push();
        // stupid but satisfies clippy
        (0..self.params.len()).for_each(|i| {
            self.closure.define(
                self.params[i].lexeme.clone(),
                arguments[i].borrow().clone(),
            );
        });
        let res = Stmt::block(self.body.clone()).execute(&mut self.closure);
        self.closure.pop();
        match res {
            ok @ Ok(_) => ok,
            Err(e) => match e {
                RuntimeError::Error { .. } => Err(e),
                RuntimeError::Return(v) => Ok(v),
            },
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
