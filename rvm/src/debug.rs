use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;

// pub fn disassemble_chunk(chunk: Chunk, name: &String) {
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    // println!("{:?} ", chunk);
    println!("== {} ==", name);

    let mut offset: usize = 0;
    while offset < chunk.codes.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    // Prints the offset (bytecode index of a chunk) with 0 padding up to 3 digits
    // https://stackoverflow.com/a/41821049
    print!("{:0width$}", offset, width = 3);

    // Print line number or | for bytecodes on the same line
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    match chunk.codes[offset] {
        OpCode::RETURN => simple_instruction("RETURN", &chunk, offset),
        OpCode::CONSTANT => constant_instruction("CONSTANT", &chunk, offset),
        OpCode::NEGATE => simple_instruction("NEGATE", &chunk, offset),
        OpCode::ConstantIndex(index) => 0, // Cos expect usize back but Const alr return 2, so this return 0
        // OpCode::ConstantIndex(index) => simple_instruction("ConstantIndex", offset),
        //
        ref instruction => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    }
}
/*
    Printing should be of the format
    Line number   OpCode index    OpCode string representation     add. data if any
*/

// fn simple_instruction(name: &str, offset: usize) -> usize {
//     println!("{}", name);
//     offset + 1
// }
fn simple_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    println!("{:?}", chunk.codes[offset]);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = &chunk.constants[offset];

    // println!("{} -> {:?}", name, constant);
    println!("{:?} -> {:?}", chunk.codes[offset], constant);
    offset + 2
}

pub fn print_stack(stack: &Vec<Value>) {
    print!("Stack [");
    for stack_value in stack {
        print!("{:?} -> ", stack_value);
    }
    // If i do this then the instructuion formatting will be off when i disassemble it
    // In VM, where debugging is printing stack then disassembled instruction
    print!("]\n");
}
