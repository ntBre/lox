use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

use clox::vm::Vm;

fn run_file(mut vm: Vm, argv: &str) {
    let source = match read_to_string(argv) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read {argv} with {e}");
            exit(74)
        }
    };

    vm.interpret(source).unwrap();
}

fn repl(mut vm: Vm) {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(n) if n == 0 => return,
            Ok(_) => (),
            Err(e) => panic!("failed to read line from stdin with '{e:?}'"),
        }

        vm.interpret(line).unwrap();
    }
}

fn main() {
    let vm = Vm::new();

    let argv: Vec<_> = args().collect();
    let argc = argv.len();

    if argc == 1 {
        repl(vm);
    } else if argc == 2 {
        run_file(vm, &argv[1]);
    } else {
        eprintln!("Usage: clox [path]");
        exit(64);
    }
}
