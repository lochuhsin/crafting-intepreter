use std::fmt::Display;

use crate::values::{GenericValue, ValueArray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    OpReturn = 1,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess, // Well, if we implement Greater Equal, Less Equal, it will be way faster, since there are only one instruction
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
            8 => OpCode::OpNil,
            9 => OpCode::OpTrue,
            10 => OpCode::OpFalse,
            11 => OpCode::OpNot,
            12 => OpCode::OpEqual,
            13 => OpCode::OpGreater,
            14 => OpCode::OpLess,
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
            OpCode::OpNil => "OP_NIL",
            OpCode::OpTrue => "OP_TRUE",
            OpCode::OpFalse => "OP_FALSE",
            OpCode::OpNot => "OP_NOT",
            OpCode::OpEqual => "OP_EQUAL_EQUAL",
            OpCode::OpGreater => "OP_GREATER",
            OpCode::OpLess => "OP_LESS",
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

    pub fn add_const(&mut self, value: GenericValue) -> usize {
        self.const_pool.write_value_array(value);
        // return the index where the constant was appended.
        self.const_pool.count - 1
    }
}
