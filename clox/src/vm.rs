//! this module contains the code for the virtual machine [Vm]. Unlike the C
//! version, it does not define a global singleton and instead defines the
//! functions that manipulate the Vm as methods on a [Vm] instance

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
    DEBUG_TRACE_EXECUTION,
};

const STACK_MAX: usize = 256;

/// use usizes instead of pointers to elements
pub struct Vm<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($self:expr, $op:tt) => {
	let b = $self.pop();
	let a = $self.pop();
	$self.push(a $op b);
    }
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            // this would actually be a prime use for maybeuninit or
            // mem::uninitialized
            stack: [Value::default(); STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(
        &mut self,
        chunk: &'a Chunk,
    ) -> Result<(), InterpretError> {
        self.chunk = Some(chunk);
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

    fn read_byte(&mut self) -> u8 {
        let res = self.chunk.unwrap().code[self.ip];
        self.ip += 1;
        res
    }

    pub(crate) fn read_constant(&mut self) -> Value {
        self.chunk.unwrap().constants[self.read_byte() as usize]
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("        ");
                for slot in self.stack.iter().take(self.stack_top) {
                    print!("[ {slot} ]");
                }
                println!();
                self.chunk.unwrap().disassemble_instruction(self.ip);
            }
            let instruction = self.read_byte();
            match instruction.try_into() {
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Ok(OpCode::Add) => {
                    binary_op!(self, +);
                }
                Ok(OpCode::Subtract) => {
                    binary_op!(self, -);
                }
                Ok(OpCode::Multiply) => {
                    binary_op!(self, *);
                }
                Ok(OpCode::Divide) => {
                    binary_op!(self, /);
                }
                Ok(OpCode::Negate) => {
                    let tmp = self.pop();
                    self.push(-tmp);
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

impl<'a> Default for Vm<'a> {
    fn default() -> Self {
        Self::new()
    }
}
