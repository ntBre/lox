use super::RuntimeError;
use super::Value;
use super::callable::Callable;
use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::token::Token;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) params: Vec<Token>,
    pub(crate) body: Vec<Stmt>,
}

impl Function {
    pub(crate) fn new(declaration: Stmt) -> Self {
        let Stmt::Function { name, params, body } = declaration else {
	        panic!("attempted to call non-function {declaration:?}");
	    };
        Self {
            name: name.lexeme,
            params,
            body,
        }
    }
}

impl Callable for Function {
     fn arity(&self) -> usize {
	self.params.len()
    }

     fn call(
        &self,
        env: &mut Environment,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        // TODO lots of cloning here, could I take self as owned? I think not
        // because then functions wouldn't be re-usable
        env.push();
        // stupid but satisfies clippy
        (0..self.params.len()).for_each(|i| {
            env.define(self.params[i].lexeme.clone(), arguments[i].clone());
        });
        let res = Stmt::block(self.body.clone()).execute(env);
        env.pop();
        res
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn todo!>")
    }
}
