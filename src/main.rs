use lolang::{
    chunk::{Chunk, OpCode},
    vm::interpret,
};

fn main() {
    let mut ch = Chunk::default();
    let constant = ch.add_const(1_f64);
    ch.write_chunk(OpCode::OpConstant as usize, 1);
    ch.write_chunk(constant, 1);
    ch.write_chunk(OpCode::OpConstant as usize, 1);
    ch.write_chunk(constant, 1);
    ch.write_chunk(OpCode::OpSubtract as usize, 1);
    ch.write_chunk(OpCode::OpNegate as usize, 1);
    ch.write_chunk(OpCode::OpReturn as usize, 1);

    interpret(&ch);
}
