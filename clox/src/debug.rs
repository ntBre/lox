use crate::chunk::{Chunk, OpCode};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub(crate) fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        let instruction = self.code[offset];
        match instruction.try_into() {
            Ok(OpCode::Constant) => {
                constant_instruction("Constant", self, offset)
            }
            Ok(OpCode::Add) => simple_instruction("Add", offset),
            Ok(OpCode::Subtract) => simple_instruction("Subtract", offset),
            Ok(OpCode::Multiply) => simple_instruction("Multiply", offset),
            Ok(OpCode::Divide) => simple_instruction("Divide", offset),
            Ok(OpCode::Negate) => simple_instruction("Negate", offset),
            Ok(OpCode::Return) => simple_instruction("Return", offset),
            Err(_) => {
                println!("Unknown opcode {instruction}");
                offset + 1
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

// this might make more sense as a method since it takes a &Chunk. could just be
// &self
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = chunk.constants[constant as usize];
    // corresponds to printValue, just rely on Display impl for Value
    println!("{name:<16} {constant:4} '{value}'");
    offset + 2
}
