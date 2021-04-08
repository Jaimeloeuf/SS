use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;

// pub fn disassemble_chunk(chunk: Chunk, name: &String) {
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====== Start of chunk: {} ======", name);

    let mut offset: usize = 0;
    while offset < chunk.codes.len() {
        offset = disassemble_instruction(&chunk, offset);
    }

    println!("====== End of chunk:   {} ======", name);
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    // Prints the offset (bytecode index of a chunk) with 0 padding up to 3 digits
    // https://stackoverflow.com/a/41821049
    print!("{:0width$}", offset, width = 3);

    /*
        Printing should be of the format
        <Line number>   <OpCode index>    <OpCode string representation>    <add. data if any>

        Print line number or | for bytecodes on the same line
    */
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    match chunk.codes[offset] {
        OpCode::POP => simple_instruction(&chunk, offset),
        OpCode::POP_N(_) => simple_instruction(&chunk, offset),
        OpCode::RETURN => simple_instruction(&chunk, offset),
        OpCode::CONSTANT(_) => constant_instruction(&chunk, offset),
        OpCode::IDENTIFIER(_) => constant_instruction(&chunk, offset),
        OpCode::IDENTIFIER_LOOKUP(_) => constant_instruction(&chunk, offset),
        OpCode::GET_LOCAL(_) => constant_instruction(&chunk, offset),
        OpCode::SET_LOCAL(_) => constant_instruction(&chunk, offset),

        OpCode::JUMP(_) => constant_instruction(&chunk, offset),
        OpCode::JUMP_IF_FALSE(_) => constant_instruction(&chunk, offset),

        OpCode::ADD => simple_instruction(&chunk, offset),
        OpCode::SUBTRACT => simple_instruction(&chunk, offset),
        OpCode::MULTIPLY => simple_instruction(&chunk, offset),
        OpCode::DIVIDE => simple_instruction(&chunk, offset),

        OpCode::NOT => simple_instruction(&chunk, offset),
        OpCode::NEGATE => simple_instruction(&chunk, offset),

        OpCode::EQUAL => simple_instruction(&chunk, offset),
        OpCode::NOT_EQUAL => simple_instruction(&chunk, offset),
        OpCode::GREATER => simple_instruction(&chunk, offset),
        OpCode::GREATER_EQUAL => simple_instruction(&chunk, offset),
        OpCode::LESS => simple_instruction(&chunk, offset),
        OpCode::LESS_EQUAL => simple_instruction(&chunk, offset),

        OpCode::PRINT => simple_instruction(&chunk, offset),

        ref instruction => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(chunk: &Chunk, offset: usize) -> usize {
    println!("{:?}", chunk.codes[offset]);
    offset + 1
}

fn constant_instruction(chunk: &Chunk, offset: usize) -> usize {
    println!("{:?}", chunk.codes[offset]);
    offset + 1
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
