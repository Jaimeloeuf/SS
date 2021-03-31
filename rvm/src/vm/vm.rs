use crate::{arithmetic_binary_op, equality_op, numeric_comparison_op};

use crate::chunk::Chunk;
use crate::debug;
use crate::error::RuntimeError;
use crate::opcode::OpCode;
use crate::value::Value;

use std::collections::HashMap;

pub struct VM {
    pub chunk: Chunk,

    // See this on why pointer is better then using a integer to access the vec
    // https://craftinginterpreters.com/a-virtual-machine.html#executing-instructions
    // ip: &'static usize,

    // ip: Instruction Pointer, points to the current bytecode instruction being executed
    // ip points to the instruction about to be executed, a.k.a the next instruction, not the current one being handled
    ip: usize,
}

impl VM {
    pub fn interpret(chunk: Chunk) -> Result<Value, RuntimeError> {
        // let mut vm = VM {
        //     chunk,
        //     // From Clox:
        //     // If we were trying to squeeze every ounce of speed out of our bytecode interpreter,
        //     // we would store ip in a local variable. It gets modified so often during execution,
        //     // that we want the compiler to keep it in a register.
        //     ip: 0,
        // };

        // @todo Include max stack to cause stack overflow to prevent infinite stack use
        // let mut top_of_stack: usize = 0; // Technically just use stack.last()
        let mut stack = Vec::<Value>::new();
        let mut values = HashMap::<String, Value>::new();

        // Add a debug flag for this
        // offset is used for disassemble_instruction
        // for ref code in &chunk.codes {
        for (offset, ref code) in chunk.codes.iter().enumerate() {
            // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose mode only
            #[cfg(debug_assertions)]
            debug::disassemble_instruction(&chunk, offset);

            match code {
                // Pop value off stack, used at the end of expression statements
                OpCode::POP => {
                    stack.pop();
                }

                // In Clox, vm access constant value in this op code by getting next byte as index and calling from const pool
                // But here value is stored in the enum variant, and is accessed directly instead of getting from a const pool
                // @todo Find a way to take value out from enum to do `stack.push(value);` instead of cloning value
                OpCode::CONSTANT(value) => stack.push(value.clone()),

                // Right now is it taking the identifier or isit copying it since identifier is a copy type?
                OpCode::IDENTIFIER(identifier) => {
                    // @todo Perhaps need to check if key is already used... should this be a runtime or compile time check
                    values.insert(identifier.clone(), stack.pop().unwrap());
                }
                OpCode::IDENTIFIER_LOOKUP(identifier) => match values.get(identifier) {
                    Some(value) => stack.push(value.clone()),
                    None => {
                        return Err(RuntimeError::UndefinedIdentifier(
                            chunk.lines[offset],
                            identifier.clone(),
                        ))
                    }
                },

                OpCode::GET_LOCAL(stack_index) => stack.push(stack[*stack_index].clone()),
                OpCode::SET_LOCAL(stack_index) => stack[*stack_index] = stack.pop().unwrap(),

                OpCode::ADD => arithmetic_binary_op!(stack, +),
                OpCode::SUBTRACT => arithmetic_binary_op!(stack, -),
                OpCode::MULTIPLY => arithmetic_binary_op!(stack, *),
                OpCode::DIVIDE => arithmetic_binary_op!(stack, /),

                OpCode::NOT => {
                    let value = stack.pop();

                    // @todo Is runtime stack value check needed?
                    // Only run this check during debug builds, assuming correctly compiled codes will not have this issue
                    #[cfg(debug_assertions)]
                    if value.is_none() {
                        panic!("VM Debug Error: Stack missing values for NOT OpCode");
                    }

                    stack.push(value.unwrap().not()?);
                }
                OpCode::NEGATE => {
                    let value = stack.pop();

                    // @todo Is runtime stack value check needed?
                    // Only run this check during debug builds, assuming correctly compiled codes will not have this issue
                    #[cfg(debug_assertions)]
                    if value.is_none() {
                        panic!("VM Debug Error: Stack missing values for NEGATE OpCode");
                    }

                    stack.push(value.unwrap().negate()?);
                }

                OpCode::EQUAL => equality_op!(stack, ==),
                OpCode::NOT_EQUAL => equality_op!(stack, !=),
                OpCode::GREATER => numeric_comparison_op!(stack, >),
                OpCode::GREATER_EQUAL => numeric_comparison_op!(stack, >=),
                OpCode::LESS => numeric_comparison_op!(stack, <),
                OpCode::LESS_EQUAL => numeric_comparison_op!(stack, <=),

                OpCode::PRINT => {
                    // @todo Dont use debug symbol
                    println!("{:?}", stack.pop().unwrap());
                }

                OpCode::RETURN => {
                    println!("RETURN:  {:?}", stack.pop().unwrap());
                }

                ref instruction => println!("VM Error: Unknown OpCode {:?}\n", instruction),
            }

            // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose mode only
            #[cfg(debug_assertions)]
            debug::print_stack(&stack);
        }

        // @todo Tmp value to return for testing
        // InterpretResult::Ok
        Ok(Value::Null)
    }
}
