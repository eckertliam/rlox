#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::OP_CONSTANT,
            1 => OpCode::OP_RETURN,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> Self {
        match op_code {
            OpCode::OP_CONSTANT => 0,
            OpCode::OP_RETURN => 1,
        }
    }
}