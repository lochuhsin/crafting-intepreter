use std::fmt::Display;

use crate::values::{Value, ValueArray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    OpReturn = 1,
    OpConstant = 2,
    OpNegate = 3,
    OpAdd = 4,
    OpSubtract = 5,
    OpMultiply = 6,
    OpDivide = 7,
}

impl OpCode {
    pub fn from_usize(value: usize) -> OpCode {
        match value {
            1 => OpCode::OpReturn,
            2 => OpCode::OpConstant,
            3 => OpCode::OpNegate,
            4 => OpCode::OpAdd,
            5 => OpCode::OpSubtract,
            6 => OpCode::OpMultiply,
            7 => OpCode::OpDivide,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            OpCode::OpReturn => "OP_RETURN",
            OpCode::OpConstant => "OP_CONST",
            OpCode::OpNegate => "OP_NEGATE",
            OpCode::OpAdd => "OP_ADD",
            OpCode::OpSubtract => "OP_SUBTRACT",
            OpCode::OpMultiply => "OP_MULTIPLY",
            OpCode::OpDivide => "OP_DIVIDE",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Chunk {
    pub bytecode: Vec<usize>,
    pub lines: Vec<usize>, // using a better implementation to store lines
    pub const_pool: ValueArray,
    pub count: usize,
}
impl Chunk {
    pub fn new(bytecode: Vec<usize>, const_pool: ValueArray, lines: Vec<usize>) -> Chunk {
        Chunk {
            count: bytecode.len(),
            bytecode,
            lines,
            const_pool,
        }
    }

    pub fn write_chunk(&mut self, bytecode: usize, line: usize) {
        self.count += 1;
        self.bytecode.push(bytecode);
        self.lines.push(line);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.const_pool.write_value_array(value);
        // return the index where the constant was appended.
        self.const_pool.count - 1
    }
}
