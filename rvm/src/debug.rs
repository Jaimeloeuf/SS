use crate::chunk::Chunk;
use crate::value::Value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====== Start of chunk: {} ======", name);
    for offset in 0..chunk.codes.len() {
        disassemble_instruction(&chunk, offset);
    }
    println!("====== End of chunk:   {} ======", name);
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
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

    println!("{:?}", chunk.codes[offset]);
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
