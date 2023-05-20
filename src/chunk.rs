use crate::value::{ValueArray, Value};
use crate::opcode::OpCode;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub values: ValueArray,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: Vec::new(),
            values: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.lines.push(line);
        self.code.push(byte);
    }

    pub fn free(&mut self) {
        self.code.clear();
        self.values.free();
        self.lines.clear();
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{} {:4} ", name, constant);
        println!("{:?}", self.values.data[constant as usize]);
        offset + 2
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if (offset > 0) && (self.lines[offset] == self.lines[offset - 1]) {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset].into();
        match instruction {
            OpCode::OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::OP_ADD => self.simple_instruction("OP_ADD", offset),
            OpCode::OP_SUBTRACT => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::OP_MULTIPLY => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::OP_DIVIDE => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::OP_NEGATE => self.simple_instruction("OP_NEGATE", offset),
            OpCode::OP_RETURN => self.simple_instruction("OP_RETURN", offset),
        }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.write_value(value);
        self.values.data.len() - 1
    }
}