use crate::chunk::Chunk;
use crate::opcode::OpCode;

pub struct VM {
    pub chunks: Vec<Chunk>,
}

// Change this to error or something
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    // CHange this to result type
    pub fn interpret(chunk: &Chunk) -> InterpretResult {
        InterpretResult::Ok
    }
}
