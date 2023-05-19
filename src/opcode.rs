#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::OP_CONSTANT,
            1 => OpCode::OP_ADD,
            2 => OpCode::OP_SUBTRACT,
            3 => OpCode::OP_MULTIPLY,
            4 => OpCode::OP_DIVIDE,
            5 => OpCode::OP_NEGATE,
            6 => OpCode::OP_RETURN,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> Self {
        match op_code {
            OpCode::OP_CONSTANT => 0,
            OpCode::OP_ADD => 1,
            OpCode::OP_SUBTRACT => 2,
            OpCode::OP_MULTIPLY => 3,
            OpCode::OP_DIVIDE => 4,
            OpCode::OP_NEGATE => 5,
            OpCode::OP_RETURN => 6,
        }
    }
}