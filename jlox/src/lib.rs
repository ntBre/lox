use std::{
    error::Error,
    fs::read_to_string,
    io::{stdout, BufRead, BufReader, Write},
};

use scanner::Scanner;

mod scanner;
mod token;
mod token_type;

type RunRes = Result<(), Box<dyn Error>>;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> RunRes {
        self.run(&read_to_string(path)?);
        if self.had_error {
            std::process::exit(65);
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
        for token in tokens {
            println!("{token}");
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, wher: &str, message: &str) {
        eprintln!("[line {line}] Error{wher}: {message}");
        self.had_error = true;
    }
}
