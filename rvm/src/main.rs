mod chunk;
mod debug;
mod error;
mod opcode;
mod value;
mod vm;

use chunk::Chunk;
use debug::disassemble_chunk;
use opcode::OpCode;
use value::Value;
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write(OpCode::CONSTANT(Value::Number(1.2)), 2);
    chunk.write(OpCode::NEGATE, 2);

    chunk.write(OpCode::CONSTANT(Value::Number(1.8)), 3);
    chunk.write(OpCode::SUBTRACT, 3);

    chunk.write(OpCode::RETURN, 3);

    disassemble_chunk(&chunk, "test");
    // println!("{:?}", chunk);

    if let Err(e) = VM::interpret(chunk) {
        println!("{}", e)
    }
}
