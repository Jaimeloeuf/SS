use std::env;
use std::fs;

mod chunk;
mod compiler;
mod debug;
mod error;
mod keywords;
mod opcode;
mod scanner;
mod token;
mod value;
mod vm;

use chunk::Chunk;
use debug::disassemble_chunk;
use opcode::OpCode;
use scanner::Scanner;
use value::Value;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Entering file '{}'", filename);

    let source = match fs::read_to_string(filename) {
        Ok(source) => source,
        Err(err) => panic!("Unable to read file: {}", err), // Bubble error up instead of panic!
    };
}

#[allow(dead_code)]
fn test_vm_with_chunk() {
    let mut chunk = Chunk::new();

    chunk.write(OpCode::CONSTANT(Value::Number(1.2)), 2);
    chunk.write(OpCode::NEGATE, 2);

    // chunk.write(OpCode::CONSTANT(Value::Null), 3);
    // chunk.write(OpCode::CONSTANT(Value::Number(4.2)), 3);
    // chunk.write(OpCode::ADD, 3);

    chunk.write(OpCode::CONSTANT(Value::Number(1.8)), 3);
    chunk.write(OpCode::SUBTRACT, 3);

    chunk.write(OpCode::RETURN, 3);

    disassemble_chunk(&chunk, "test");
    // println!("{:?}", chunk);

    if let Err(e) = VM::interpret(chunk) {
        println!("{}", e)
    }
}
