//! this module contains the code for the virtual machine [Vm]. Unlike the C
//! version, it does not define a global singleton and instead defines the
//! functions that manipulate the Vm as methods on a [Vm] instance

use crate::{
    chunk::{Chunk, OpCode},
    compile::Parser,
    value::Value,
    DEBUG_TRACE_EXECUTION,
};

const STACK_MAX: usize = 256;

/// use usizes instead of pointers to elements
pub struct Vm {
    pub(crate) chunk: Option<Chunk>,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
    pub(crate) parser: Parser,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($self:expr, $op:tt, $typ:ident) => {
	if !$self.peek(0).is_number() || !$self.peek(1).is_number() {
	    $self.runtime_error("Operands must be numbers.");
	    return Err(InterpretError::RuntimeError);
	}
	let b = $self.pop();
	let b = b.as_number().unwrap();
	let a = $self.pop();
	let a = a.as_number().unwrap();
	$self.push(Value::$typ(a $op b));
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            // this would actually be a prime use for maybeuninit or
            // mem::uninitialized
            stack: [Value::default(); STACK_MAX],
            stack_top: 0,
            parser: Parser::default(),
        }
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    fn runtime_error(&mut self, msg: &str) {
        eprintln!("{msg}");

        let instruction = self.ip - 1;
        let line = self.chunk.as_ref().unwrap().lines[instruction];
        eprintln!("[line {line} in script\n");
        self.reset_stack();
    }

    pub fn interpret(&mut self, source: String) -> Result<(), InterpretError> {
        let chunk = self.compile(source)?;
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        std::mem::take(&mut self.stack[self.stack_top])
    }

    /// peek the value at `distance` from the top of the stack
    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack_top - 1 - distance]
    }

    fn read_byte(&mut self) -> u8 {
        let res = self.chunk.as_mut().unwrap().code[self.ip];
        self.ip += 1;
        res
    }

    pub(crate) fn read_constant(&mut self) -> Value {
        let b = self.read_byte();
        self.chunk.as_mut().unwrap().constants[b as usize]
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("        ");
                for slot in self.stack.iter().take(self.stack_top) {
                    print!("[ {slot} ]");
                }
                println!();
                self.chunk
                    .as_mut()
                    .unwrap()
                    .disassemble_instruction(self.ip);
            }
            let instruction = self.read_byte();
            match instruction.try_into() {
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Ok(OpCode::Nil) => self.push(Value::nil()),
                Ok(OpCode::True) => self.push(Value::boolean(true)),
                Ok(OpCode::False) => self.push(Value::boolean(false)),
                Ok(OpCode::Equal) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::boolean(a.eq(&b)));
                }
                Ok(OpCode::Greater) => {
                    binary_op!(self, >, Bool);
                }
                Ok(OpCode::Less) => {
                    binary_op!(self, <, Bool);
                }
                Ok(OpCode::Add) => {
                    binary_op!(self, +, Number);
                }
                Ok(OpCode::Subtract) => {
                    binary_op!(self, -, Number);
                }
                Ok(OpCode::Multiply) => {
                    binary_op!(self, *, Number);
                }
                Ok(OpCode::Divide) => {
                    binary_op!(self, /, Number);
                }
                Ok(OpCode::Not) => {
                    let tmp = self.pop();
                    self.push(Value::boolean(tmp.is_falsey()));
                }
                Ok(OpCode::Negate) => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number");
                        return Err(InterpretError::RuntimeError);
                    }
                    let tmp = self.pop();
                    let tmp = tmp.as_number().unwrap();
                    self.push(Value::number(-tmp));
                }
                Ok(OpCode::Return) => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                Err(_) => todo!(),
            }
        }
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
