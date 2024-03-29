use super::callable::Callable;
use super::Interpreter;
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
        int: &mut Interpreter,
        arguments: Vec<Rc<RefCell<Value>>>,
    ) -> Result<Rc<RefCell<Value>>, RuntimeError> {
        // clone the outer environment, append the closure's stack to it, then
        // call function. restore the closure at the end
        let mut env = int.globals.clone();
        let start = env.stack.len();
        env.stack.extend(std::mem::take(&mut self.closure.stack));
        env.push();
        // stupid but satisfies clippy
        (0..self.params.len()).for_each(|i| {
            env.define(
                self.params[i].lexeme.clone(),
                arguments[i].borrow().clone(),
            );
        });
        let tmp = std::mem::take(&mut int.globals);
        int.globals = env;
        let res = int.execute(Stmt::block(self.body.clone()));
        int.globals.pop();
        self.closure.stack = int.globals.stack[start..].to_owned();
        int.globals = tmp;
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
