mod chunk;
mod debug;
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

    chunk.write(OpCode::CONSTANT, 2);
    chunk.constants.push(Value::Number(1.2));
    chunk.write(OpCode::ConstantIndex(chunk.constants.len()), 2);

    chunk.write(OpCode::RETURN, 2);

    disassemble_chunk(&chunk, "test");
    // println!("{:?}", chunk);
}
