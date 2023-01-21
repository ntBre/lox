use crate::{scanner::Scanner, vm::Vm};

impl<'a> Vm<'a> {
    pub(crate) fn compile(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let mut line = 0;
        loop {
            let token = scanner.scan_token();
            if token.line != line {
                print!("{:4}", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!(
                "{:2} '{:width$}'",
                token.typ,
                token.start,
                width = token.length
            );

            if token.typ.is_eof() {
                break;
            }
        }
    }
}
