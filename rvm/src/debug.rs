use crate::chunk::Chunk;
use crate::opcode::OpCode;

// pub fn disassemble_chunk(chunk: Chunk, name: &String) {
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    // println!("{:?} ", chunk);
    println!("== {} ==", name);

    let mut offset: usize = 0;
    while offset < chunk.codes.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
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
        // OpCode::ConstantIndex(index) => simple_instruction("ConstantIndex", offset),
        //
        ref instruction => {
            println!("Unknown opcode {:?}\n", instruction);
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
