#![allow(unused)]

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
use interpreter::{builtin::Builtin, value::Value, Interpreter, RuntimeError};
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
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
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

        let mut interpreter = Interpreter::new(self);

        // let mut resolver = Resolver::new(&mut interpreter);
        // resolver.resolve(&statements);
	if interpreter.lox.had_error {
	    return;
	}

        interpreter.interpret(statements);
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
