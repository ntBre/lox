use std::cmp::Ordering;

use jlox::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lox = Lox::new();
    // 2 instead of 1 in the book because of the executable name
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
        Ordering::Equal => lox.run_file(&args[1]).unwrap(),
        Ordering::Less => lox.run_prompt().unwrap(),
    }
}
