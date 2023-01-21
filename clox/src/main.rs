use clox::{
    chunk::{Chunk, OpCode},
    vm::Vm,
};

fn main() {
    let mut vm = Vm::new();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::Constant, 123);
    chunk.write_chunk(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::Constant, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::Add, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::Constant, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::Divide, 123);
    chunk.write_chunk(OpCode::Negate, 123);

    chunk.write_chunk(OpCode::Return, 123);

    vm.interpret(&chunk).unwrap();
}
