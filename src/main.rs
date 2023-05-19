mod chunk;
mod value;
mod opcode;
mod vm;


use value::Value;
use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    let mut vm = vm::VM::new();
    let mut constant = chunk.add_constant(Value::Number(1.2));
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    constant = chunk.add_constant(Value::Number(3.4));
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OpCode::OP_ADD as u8, 123);
    constant = chunk.add_constant(Value::Number(5.6));
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OpCode::OP_DIVIDE as u8, 123);
    chunk.write_chunk(OpCode::OP_NEGATE as u8, 123);
    chunk.write_chunk(OpCode::OP_RETURN as u8, 123);
    vm.debug_interpret(chunk);
}
