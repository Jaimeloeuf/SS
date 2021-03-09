use crate::chunk::Chunk;
use crate::debug;
use crate::error::RuntimeError;
use crate::opcode::OpCode;
use crate::value::Value;

pub struct VM {
    pub chunk: Chunk,

    // See this on why pointer is better then using a integer to access the vec
    // https://craftinginterpreters.com/a-virtual-machine.html#executing-instructions
    // ip: &'static usize,

    // ip: Instruction Pointer, points to the current bytecode instruction being executed
    // ip points to the instruction about to be executed, a.k.a the next instruction, not the current one being handled
    ip: usize,
}

// Change this to error or something
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    // pub fn interpret(chunk: Chunk) -> InterpretResult {
    pub fn interpret(mut chunk: Chunk) -> Result<Value, RuntimeError> {
        // let mut vm = VM {
        //     chunk,
        //     ip: 0,
        // };

        // @todo Include max stack to cause stack overflow to prevent infinite stack use
        // let mut top_of_stack: usize = 0; // Technically just use stack.last()
        // let mut stack = Vec::<&Value>::new();
        let mut stack = Vec::<Value>::new();

        // Add a debug flag for this
        // offset is used for disassemble_instruction
        // for ref code in &chunk.codes {
        for (offset, ref code) in chunk.codes.iter().enumerate() {
            // Add a debug flag for this
            {
                debug::print_stack(&stack);
                debug::disassemble_instruction(&chunk, offset);
            }

            match code {
                // In Clox, vm access constant value in this op code by getting next byte as index and calling from const pool
                // But here value is stored in the enum variant, and is accessed directly instead of getting from a const pool
                // @todo Find a way to take value out from enum instead of cloning value stack.push(value);
                OpCode::CONSTANT(value) => stack.push(value.clone()),

                OpCode::NEGATE => {
                    let value = stack.pop().unwrap().negate()?;
                    stack.push(value);
                }

                OpCode::RETURN => {
                    println!("{:?}", stack.pop().unwrap());
                    // println!("break!");
                    // return InterpretResult::Ok;
                }

                ref instruction => println!("VM: Unknown OpCode {:?}\n", instruction),
            }
        }

        // @todo Tmp value to return for testing
        // InterpretResult::Ok
        Ok(Value::Null)
    }
}
