use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::constants;
use crate::value::Value;
// Note: Change this, don't use global variables

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRunTimeError,
}

#[derive(Default)]
pub struct VirtualMachine {
    pub chunk: Chunk,
    pub ip: usize, // instruction pointer, the index currently pointing to the instruction in chunk
    pub vm_stack: VirtualMachineStack,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk) -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            chunk,
            vm_stack: VirtualMachineStack::default(),
        }
    }
    pub fn update_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
    }
}

pub fn interpret(chunk: &Chunk) -> InterpretResult {
    let mut vm = VirtualMachine::new(chunk.clone());
    run(&mut vm)
}

pub fn run(vm: &mut VirtualMachine) -> InterpretResult {
    loop {
        #[cfg(debug_assertions)]
        {
            for i in 0..vm.vm_stack.top {
                print!("[ {} ]", vm.vm_stack.values[i])
            }
            println!();
            disassemble_instruction(&vm.chunk, vm.ip);
        }
        let op_code = read_op(vm);
        match op_code {
            OpCode::OpConstant => {
                let val = read_constant(vm);
                vm.vm_stack.push(val);
            }
            OpCode::OpReturn => {
                println!("{}", vm.vm_stack.pop());
                return InterpretResult::InterpretOk;
            }
            OpCode::OpNegate => {
                let v = -vm.vm_stack.pop();
                vm.vm_stack.push(v);
            }
        };
    }
}

fn read_op_raw(vm: &mut VirtualMachine) -> usize {
    let code = vm.chunk.bytecode[vm.ip];
    vm.ip += 1;
    code
}

fn read_op(vm: &mut VirtualMachine) -> OpCode {
    let code = vm.chunk.bytecode[vm.ip];
    vm.ip += 1;
    OpCode::from_usize(code)
}

fn read_constant(vm: &mut VirtualMachine) -> Value {
    let code = read_op_raw(vm);
    vm.chunk.const_pool.values[code]
}

////////////////////////////////////////////////////////////////
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset)
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
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
        OpCode::OpNegate => simple_instruction(instruction, offset),
        _ => {
            println!("Unknown instruction");
            offset + 1
        }
    }
}

pub fn simple_instruction(op: OpCode, offset: usize) -> usize {
    println!("{}", op);
    offset + 1
}

pub fn constant_instruction(op: OpCode, offset: usize, chunk: &Chunk) -> usize {
    let constant = chunk.bytecode[offset + 1];
    let val = chunk.const_pool.values[constant];

    println!("{}{}'{}'", op, " ".repeat(15), val);
    offset + 2
}

pub struct VirtualMachineStack {
    pub values: [Value; constants::STACK_MAX as usize],
    pub top: usize,
}

impl VirtualMachineStack {
    pub fn push(&mut self, value: Value) {
        if self.top >= self.values.len() {
            panic!("Invalid operation, exceeds stack limit")
        }
        self.values[self.top] = value;
        self.top += 1;
    }

    pub fn pop(&mut self) -> Value {
        if self.top == 0 {
            panic!("Invalid operation, empty stack ")
        }
        self.top -= 1;
        self.values[self.top]
    }
}

impl Default for VirtualMachineStack {
    fn default() -> Self {
        VirtualMachineStack {
            values: [0.0; constants::STACK_MAX as usize],
            top: 0,
        }
    }
}
