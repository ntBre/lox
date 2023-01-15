use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fs::read_to_string,
    io::{stdout, BufRead, BufReader, Write},
    rc::Rc,
};

use environment::Environment;
use expr::Expr;
use interpreter::{builtin::Builtin, RuntimeError, Value};
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use stmt::Stmt;
use token::Token;

mod environment;
mod expr;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;
mod token_type;

type RunRes = Result<(), Box<dyn Error>>;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    environment: Environment,
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

impl Lox {
    pub fn new() -> Self {
        let mut environment = Environment::new();
        environment.define(
            "clock".to_owned(),
            Value::Builtin(Builtin {
                params: Vec::new(),
                fun: clock,
            }),
        );
        Self {
            had_error: false,
            had_runtime_error: false,
            environment,
            locals: HashMap::new(),
        }
    }

    fn resolve(&mut self, expr: Expr, depth: usize) {
	self.locals.insert(expr, depth);
    }

    /// NOTE defining this and the `environment` on self instead of defining an
    /// Interpreter struct. I think the Java version needs that because of the
    /// Visitor pattern and I can't see how to make it work with Rust lifetimes
    /// because the Interpreter needs a mutable reference to Lox itself for
    /// errors
    fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            if let Err(e) = statement.execute(&mut self.environment) {
                self.runtime_error(e);
            }
        }
    }

    pub fn run_file(&mut self, path: &str) -> RunRes {
        self.run(&read_to_string(path)?);
        if self.had_error {
            std::process::exit(65);
        }
        if self.had_runtime_error {
            std::process::exit(70);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> RunRes {
        let mut input = BufReader::new(std::io::stdin());
        let mut line = String::new();
        loop {
            print!("> ");
            stdout().flush().unwrap();
            // okay to return on this error because it means there was an error
            // reading from stdin, not a language error
            match input.read_line(&mut line) {
                Ok(n) if n == 0 => return Ok(()),
                Ok(_) => {}
                Err(err) => return Err(Box::new(err)),
            };
            self.run(&line);
            self.had_error = false;
            line.clear();
        }
    }

    fn run(&mut self, s: &str) {
        let mut scanner = Scanner::new(s.to_owned(), self);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens, self);
        let statements = parser.parse();

        if self.had_error {
            return;
        }

        // let mut resolver = Resolver::new(self);
        // resolver.resolve(&statements);

        self.interpret(statements);
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, wher: &str, message: &str) {
        eprintln!("[line {line}] Error{wher}: {message}");
        self.had_error = true;
    }

    fn parse_error(&mut self, token: Token, message: &str) {
        if token.typ.is_eof() {
            self.report(token.line, " at end", message);
        } else {
            self.report(
                token.line,
                &format!(" at '{}'", token.lexeme),
                message,
            );
        }
    }

    fn runtime_error(&mut self, error: RuntimeError) {
        eprintln!("{}\n[line {}]", error.message(), error.line());
        self.had_runtime_error = true;
    }
}
