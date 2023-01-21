use clox::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::Constant, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::Return, 123);
    chunk.disassemble("test chunk");
}
