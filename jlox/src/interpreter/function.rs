use super::callable::Callable;
use super::RuntimeError;
use super::Value;
use crate::environment::Environment;
use crate::stmt::Stmt;
use crate::token::Token;
use std::fmt::Display;

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

    /// TODO lots of cloning here, could I take self as owned? I think not
    /// because then functions wouldn't be re-usable
    fn call(
        &mut self,
        _env: &mut Environment,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        // BUG this is not right, but i'm not sure how to fix it yet. this takes
        // as the parent environment the environment at the time of definition,
        // not at the time of call. using the passed environment would instead
        // use the environment at the time of call and ignore that at the time
        // of declaration. what we really need is some way to unify these that
        // isn't just tacking one on to the other, as I tried first

        // BUG the real issue is that I clone on env lookups, so every instance
        // of the closure call gets its own pristine version from the original
        // environment. it's not really possible to fix this I think because it
        // basically involves returning a mutable reference from env.get, which
        // means Value will have to hold a &'something value, at least for
        // functions, but I can't have a different `get` impl for functions, so
        // it has to hold a &'something to every variant.
        // println!("incoming = {:?}", self.closure);
        self.closure.push();
        // stupid but satisfies clippy
        (0..self.params.len()).for_each(|i| {
            self.closure
                .define(self.params[i].lexeme.clone(), arguments[i].clone());
        });
        let res = Stmt::block(self.body.clone()).execute(&mut self.closure);
        // println!("res = {:?}", self.closure);
        self.closure.pop();
        // println!("pop = {:?}", self.closure);
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
        write!(f, "<fn todo!>")
    }
}
