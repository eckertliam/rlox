use crate::opcode::OpCode;
use crate::chunk::Chunk;
use crate::value::Value;
use crate::compiler::compile;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

// BINARY_OP macro
macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        {
            let b = $self.pop();
            let a = $self.pop();
            $self.push(a $op b);
        }
    };
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: [Value::Number(0.0); STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn stack_trace(&self) {
        for i in (0..self.stack_top).rev() {
            println!("[{:?}]", self.stack[i]);
        }
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }
    
    pub fn interpret(&mut self, source: String) -> InterpretResult {
        self.chunk = compile(source);
        self.ip = 0;
        self.run()
    }

    // run with stack trace
    // TODO: implement compiler flag to enable/disable stack trace
    pub fn debug_interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.debug_run()
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let constant = self.read_byte();
        self.chunk.values.data[constant as usize]
    }

    fn debug_run(&mut self) -> InterpretResult {
        loop {
            self.stack_trace();
            let instruction: OpCode = self.read_byte().into();

            match instruction {
                OpCode::OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::OP_ADD => binary_op!(self, +),
                OpCode::OP_SUBTRACT => binary_op!(self, -),
                OpCode::OP_MULTIPLY => binary_op!(self, *),
                OpCode::OP_DIVIDE => binary_op!(self, /),
                OpCode::OP_NEGATE => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::OP_RETURN => {
                    println!("{:?}", self.pop());
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction: OpCode = self.read_byte().into();

            match instruction {
                OpCode::OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::OP_ADD => binary_op!(self, +),
                OpCode::OP_SUBTRACT => binary_op!(self, -),
                OpCode::OP_MULTIPLY => binary_op!(self, *),
                OpCode::OP_DIVIDE => binary_op!(self, /),
                OpCode::OP_NEGATE => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::OP_RETURN => {
                    println!("{:?}", self.pop());
                    return InterpretResult::Ok;
                }
            }
        }
    }
}