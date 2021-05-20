use crate::{arithmetic_binary_op, equality_op, numeric_comparison_op};

use crate::chunk::Chunk;
use crate::debug;
use crate::error::RuntimeError;
use crate::opcode::OpCode;
use crate::value::Value;
use crate::SSError;

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
    // Wrapper method over the inner hidden _interpret method, to wrap any RuntimeError as SSError before bubbling it up
    // @todo Instead use a trait on SSError/RuntimeError to convert between each other
    pub fn interpret(chunk: Chunk) -> Result<Value, SSError> {
        match VM::_interpret(chunk) {
            Ok(value) => Ok(value),
            Err(e) => Err(SSError::RuntimeError(e)),
        }
    }

    fn _interpret(chunk: Chunk) -> Result<Value, RuntimeError> {
        // let mut vm = VM {
        //     chunk,
        //     // From Clox:
        //     // If we were trying to squeeze every ounce of speed out of our bytecode interpreter,
        //     // we would store ip in a local variable. It gets modified so often during execution,
        //     // that we want the compiler to keep it in a register.
        //     ip: 0,
        // };

        // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose/vm-debugging mode only
        #[cfg(debug_assertions)]
        println!("Chunk opcodes: {:#?}", chunk.codes);

        // Local variable Instruction Pointer is a array index pointing to the current OpCode in chunk's 'codes' vector
        let mut ip: usize = 0;

        // @todo Include max stack to cause stack overflow to prevent infinite stack use
        // let mut top_of_stack: usize = 0; // Technically just use stack.last()
        // @todo Look into --> https://docs.rs/smallvec/1.6.1/smallvec/
        let mut stack = Vec::<Value>::new();
        let mut values = HashMap::<String, Value>::new();

        // Call stack for function calls in SS.
        // For now the call stack only stores the opcode_index to return to for execution, which is the ip value when a call opcode is executed
        let mut call_stack = Vec::<usize>::new();

        // Keep looping and executing as long as Instruction Pointer does not point past the length of codes in current chunk
        while ip < chunk.codes.len() {
            // Get ref to current OpCode in chunk to execute
            // Taking ref instead of moving it out as the code might still be executed again, e.g. in loops
            let code = &chunk.codes[ip];

            // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose mode only
            #[cfg(debug_assertions)]
            debug::disassemble_instruction(&chunk, ip);

            match code {
                // Pop value off stack, used at the end of expression statements
                OpCode::POP => {
                    // Runtime check on debug builds to ensure number of pops less than number of values on stack
                    #[cfg(debug_assertions)]
                    if stack.len() == 0 {
                        panic!(format!(
                            "VM Debug Error: Attempt to pop value from empty Stack"
                        ));
                    }

                    stack.pop();
                }
                // Pop N number of values off stack, usually used to pop local values off stack when local scope ends
                OpCode::POP_N(number_of_pops) => {
                    // Runtime check on debug builds to ensure number of pops less than number of values on stack
                    #[cfg(debug_assertions)]
                    if stack.len() < *number_of_pops {
                        panic!(format!(
                            "VM Debug Error: Popping {} values from Stack of {} values",
                            number_of_pops,
                            stack.len()
                        ));
                    }

                    // https://stackoverflow.com/questions/28952411/what-is-the-idiomatic-way-to-pop-the-last-n-elements-in-a-mutable-vec
                    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate
                    // https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_sub
                    stack.truncate(stack.len() - number_of_pops);
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
                            chunk.lines[ip],
                            identifier.clone(),
                        ))
                    }
                },

                OpCode::GET_LOCAL(stack_index) => stack.push(stack[*stack_index].clone()),
                OpCode::SET_LOCAL(stack_index) => stack[*stack_index] = stack.pop().unwrap(),

                OpCode::TYPE_CHECK_BOOL => {
                    let value = stack.last();

                    // @todo Is runtime stack value check needed?
                    // Only run this check during debug builds, assuming correctly compiled codes will not have this issue
                    #[cfg(debug_assertions)]
                    if value.is_none() {
                        panic!("VM Debug Error: Stack missing value for TYPE_CHECK_BOOL OpCode");
                    }

                    match value {
                        Some(Value::Bool(_)) => {}

                        // @todo Fix the error message
                        // Runtime type checking
                        Some(invalid_type) => {
                            return Err(RuntimeError::TypeError("Expect Boolean".to_string()))
                        }

                        // This should be a no value error
                        None => todo!(),
                    }
                }

                OpCode::CALL => {
                    match stack.pop() {
                        Some(Value::Fn(opcode_index)) => {
                            // Calculate the return opcode index after function body executes a return instruction
                            // EITHER set as ip + 1 here and return set ip = caller_ip before calling continue to skip end of loop ip increment
                            // OR set to ip, then return set ip = caller_ip, before using end of loop increment of 1
                            call_stack.push(ip + 1);

                            // Set ip to the opcode index of the function body, so that in the next loop, this will execute the first instruction of the function body
                            ip = opcode_index;

                            // To skip rest of the loop body, skipping the ip increment code
                            continue;
                        }

                        // @todo Fix the error message
                        // Runtime type checking
                        Some(invalid_type) => {
                            // panic!("VM tmp Error: Not a function");
                            return Err(RuntimeError::TypeError("Expect Function".to_string()));
                        }

                        // @todo This should be a no value error
                        None => todo!(),
                    }
                }

                OpCode::JUMP(offset) => {
                    ip += offset;
                    continue; // To skip rest of the loop body, skipping the ip increment code
                }
                OpCode::JUMP_IF_FALSE(offset) => {
                    // Dont pop the value off the stack, just take a ref to it
                    // POP instructions will be generated seperately
                    let value = stack.last();

                    // @todo Is runtime stack value check needed?
                    // Only run this check during debug builds, assuming correctly compiled codes will not have this issue
                    #[cfg(debug_assertions)]
                    if value.is_none() {
                        panic!("VM Debug Error: Stack missing value for JUMP_IF_FALSE OpCode");
                    }

                    match value {
                        // Only handle bool cases
                        Some(Value::Bool(bool)) => {
                            // Only offset VM's ip if condition evaluates to false, to skip the codes for 'true branch'
                            if *bool == false {
                                ip += offset;
                                continue; // To skip rest of the loop body, skipping the ip increment code
                            }
                        }

                        // @todo Fix the error message
                        // Runtime type checking
                        Some(invalid_type) => {
                            return Err(RuntimeError::TypeError("Expect Boolean".to_string()))
                        }

                        // This should be a no value error
                        None => todo!(),
                    }
                }

                OpCode::LOOP(offset) => {
                    ip -= offset;

                    /*
                        The offset is already calculated so that 'ip - offset' is the start of the loop conditional expression.
                        So the rest of the loop body must be skipped with continue, to skip the ip increment code.

                        Q:  Why not calculate offset to be 'offset + 1' so that this will work with the ip increment?
                        A:  The edge case where the start of the loop conditional expression is ip == 0 prevents it,
                            because ip is of usize type, which means 'ip - offset' cannot be less than 0,
                            which means that 'ip - offset + 1' will always be at least 1, break the code in this edge case.
                    */
                    continue;
                }

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
                        panic!("VM Debug Error: Stack missing value for NOT OpCode");
                    }

                    stack.push(value.unwrap().not()?);
                }
                OpCode::NEGATE => {
                    let value = stack.pop();

                    // @todo Is runtime stack value check needed?
                    // Only run this check during debug builds, assuming correctly compiled codes will not have this issue
                    #[cfg(debug_assertions)]
                    if value.is_none() {
                        panic!("VM Debug Error: Stack missing value for NEGATE OpCode");
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
                    println!("RETURN - value:  {:?}", stack.last());

                    // Get opcode index of function caller to set as ip, to resume execution at call site
                    ip = call_stack.pop().unwrap();

                    // To skip rest of the loop body, skipping the ip increment code
                    continue;
                }

                #[allow(unreachable_patterns)]
                ref instruction => println!("VM Error: Unknown OpCode {:?}\n", instruction),
            }

            // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose mode only
            #[cfg(debug_assertions)]
            debug::print_stack(&stack);

            // Increment ip (Instruction Pointer) by 1 on every loop
            ip += 1;
        }

        // @todo Tmp value to return for testing
        // InterpretResult::Ok
        Ok(Value::Null)
    }
}
