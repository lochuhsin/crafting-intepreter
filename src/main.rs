use lolang::chunk::{Chunk, OpCode};

fn main() {
    let mut ch = Chunk::default();
    // constant instruction
    let constant = ch.add_const(1.2);
    ch.write_chunk(OpCode::OpConstant as usize, 123);
    ch.write_chunk(constant, 123);
    // simple instruction
    ch.write_chunk(OpCode::OpReturn as usize, 123);

    disassemble_chunk(&ch, "test instructions");
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset)
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04}   ", offset);

    let instruction = OpCode::from_usize(chunk.bytecode[offset]);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!(" |     ")
    } else {
        print!("{:04}   ", chunk.lines[offset])
    }

    match instruction {
        OpCode::OpReturn => simple_instruction(instruction, offset),
        OpCode::OpConstant => constant_instruction(instruction, offset, chunk),
        _ => {
            println!("Unknown instruction");
            offset + 1
        }
    }
}

fn simple_instruction(op: OpCode, offset: usize) -> usize {
    println!("{}", op);
    offset + 1
}

fn constant_instruction(op: OpCode, offset: usize, chunk: &Chunk) -> usize {
    let constant = chunk.bytecode[offset + 1];
    let val = chunk.const_pool.values[constant];

    println!("{}{}'{}'", op, " ".repeat(15), val);
    offset + 2
}
