use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
    vm::{InterpretError, Vm},
    DEBUG_PRINT_CODE,
};

#[derive(Default)]
pub(crate) struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    #[default]
    None = 0,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        use Precedence::*;
        match value {
            0 => None,
            1 => Assignment,
            2 => Or,
            3 => And,
            4 => Equality,
            5 => Comparison,
            6 => Term,
            7 => Factor,
            8 => Unary,
            9 => Call,
            10 => Primary,
            _ => panic!(),
        }
    }
}

// this doesn't feel like it's going to work but it might
type ParseFn = for<'a, 'b> fn(&'a mut Vm, &'b mut Scanner);

#[derive(Default, Clone)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

fn load_rules() -> Vec<ParseRule> {
    let mut rules = vec![ParseRule::default(); 40];
    include!("rules");
    rules
}

lazy_static::lazy_static! {
    static ref RULES: Vec<ParseRule> = load_rules();
}

fn get_rule(typ: TokenType) -> &'static ParseRule {
    &RULES[typ as u8 as usize]
}

impl Vm {
    pub(crate) fn compile(
        &mut self,
        source: String,
    ) -> Result<Chunk, InterpretError> {
        let chunk = Chunk::new();
        let mut scanner = Scanner::new(source);

        self.chunk = Some(chunk);

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance(&mut scanner);
        self.expression(&mut scanner);
        self.consume(TokenType::Eof, "Expect end of expression.", &mut scanner);

        self.end();

        if self.parser.had_error {
            Err(InterpretError::CompileError)
        } else {
            Ok(std::mem::take(&mut self.chunk.as_mut().unwrap()))
        }
    }

    fn advance(&mut self, scanner: &mut Scanner) {
        self.parser.previous = std::mem::take(&mut self.parser.current);

        loop {
            self.parser.current = scanner.scan_token();
            if self.parser.current.typ != TokenType::Error {
                break;
            }

            self.error_at_current(&scanner.get_token(&self.parser.current));
        }
    }

    fn consume(
        &mut self,
        typ: TokenType,
        message: &str,
        scanner: &mut Scanner,
    ) {
        if self.parser.current.typ == typ {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.line;
        self.current_chunk().write_chunk(byte, line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub(crate) fn end(&mut self) {
        self.emit_return();
        if DEBUG_PRINT_CODE && !self.parser.had_error {
            self.current_chunk().disassemble("code");
        }
    }

    fn binary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.parser.previous.typ;
        let rule = get_rule(operator_type);
        self.parse_precedence(
            Precedence::from(rule.precedence as u8 + 1),
            scanner,
        );

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.",
            scanner,
        );
    }

    fn number(&mut self, scanner: &mut Scanner) {
        let value = scanner
            .get_token(&self.parser.previous)
            .parse::<f64>()
            .unwrap();
        self.emit_constant(value);
    }

    fn unary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.parser.previous.typ;

        // compile the operand
        self.parse_precedence(Precedence::Unary, scanner);

        let TokenType::Minus = operator_type else {
	    unreachable!();
	};

        self.emit_byte(OpCode::Negate as u8);
    }

    fn parse_precedence(
        &mut self,
        precedence: Precedence,
        scanner: &mut Scanner,
    ) {
        self.advance(scanner);
        let prefix_rule = get_rule(self.parser.previous.typ).prefix;
        let Some(rule) = prefix_rule else {
	    self.error("Expect expression.");
	    return;
	};

        rule(self, scanner);

        while precedence <= get_rule(self.parser.current.typ).precedence {
            self.advance(scanner);
            (get_rule(self.parser.previous.typ).infix.unwrap())(self, scanner);
        }
    }

    pub(crate) fn emit_constant(&mut self, value: Value) {
        let c = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, c);
    }

    pub(crate) fn make_constant(&mut self, value: Value) -> u8 {
        // apparently it's impossible to construct a constant that is too large
        // and we can't check it
        self.current_chunk().add_constant(value)
    }

    pub(crate) fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().unwrap()
    }

    pub(crate) fn expression(&mut self, scanner: &mut Scanner) {
        self.parse_precedence(Precedence::Assignment, scanner);
    }

    fn error(&mut self, message: &str) {
        let tok = self.parser.previous.clone();
        self.error_at(&tok, message);
    }

    fn error_at_current(&mut self, message: &str) {
        // this is stupid, but this whole thing is a bit stupid
        let tok = self.parser.current.clone();
        self.error_at(&tok, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.typ.is_eof() {
            eprint!(" at end");
        } else if token.typ.is_error() {
            // nothing
        } else {
            eprint!(" at '{message}'");
        }
        self.parser.had_error = true;
    }
}
