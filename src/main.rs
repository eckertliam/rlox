mod chunk;
mod value;
mod opcode;


use value::Value;
use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write_chunk(OpCode::OP_CONSTANT.into(), 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OpCode::OP_RETURN.into(), 123);
    chunk.disassemble("test chunk");
}
