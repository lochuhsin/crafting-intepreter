use lolang::{
    chunk::{Chunk, OpCode},
    vm::{disassemble_chunk, interpret},
};

fn main() {
    let mut ch = Chunk::default();
    let constant = ch.add_const(1.2);
    ch.write_chunk(OpCode::OpConstant as usize, 123);
    ch.write_chunk(constant, 123);
    ch.write_chunk(OpCode::OpNegate as usize, 123);
    ch.write_chunk(OpCode::OpReturn as usize, 123);

    interpret(&ch);
}
