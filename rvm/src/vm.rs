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

macro_rules! arithmetic_binary_op {
    // $perator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    // $stack ->  Takes in the identifier for the stack value too
    ($operator:tt, $stack:ident) => {{
        let b = $stack.pop();
        let a = $stack.pop();

        // Only run this check during debug builds, assuming correctly generated OpCodes will not have this issue
        #[cfg(debug_assertions)]
        if a.is_none() || b.is_none(){
            panic!(format!("VM Error: Stack missing values for arithmetic binary operation {}", stringify!($operator)));
        }

        match (a, b) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => {
                $stack.push(Value::Number(a $operator b));
            }

            (a, b)=> {
                // Unwrap the values directly assuming that they are definitely Some() variants
                // If it fails, it means opcodes are generated wrongly where the stack is missing values needed for the opcode
                return Err(RuntimeError::TypeError(format!(
                    "Invalid operand types {:?} and {:?} used for '{}' arithmetic operation",
                    a.unwrap(), b.unwrap(), stringify!($operator)
                )))
            }
        }
    }};
}

impl VM {
    // pub fn interpret(chunk: Chunk) -> InterpretResult {
    pub fn interpret(mut chunk: Chunk) -> Result<Value, RuntimeError> {
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
        // let mut stack = Vec::<&Value>::new();
        let mut stack = Vec::<Value>::new();

        // Add a debug flag for this
        // offset is used for disassemble_instruction
        // for ref code in &chunk.codes {
        for (offset, ref code) in chunk.codes.iter().enumerate() {
            // Only do this for debug builds, might add additonal debug flag to run this in vm-verbose mode only
            #[cfg(debug_assertions)]
            {
                debug::print_stack(&stack);
                debug::disassemble_instruction(&chunk, offset);
            }

            match code {
                // In Clox, vm access constant value in this op code by getting next byte as index and calling from const pool
                // But here value is stored in the enum variant, and is accessed directly instead of getting from a const pool
                // @todo Find a way to take value out from enum to do `stack.push(value);` instead of cloning value
                OpCode::CONSTANT(value) => stack.push(value.clone()),

                OpCode::ADD => arithmetic_binary_op!(+, stack),
                OpCode::SUBTRACT => arithmetic_binary_op!(-, stack),
                OpCode::MULTIPLY => arithmetic_binary_op!(*, stack),
                OpCode::DIVIDE => arithmetic_binary_op!(/, stack),

                OpCode::NEGATE => {
                    let value = stack.pop().unwrap().negate()?;
                    stack.push(value);
                }

                OpCode::RETURN => {
                    println!("{:?}", stack.pop().unwrap());
                }

                ref instruction => println!("VM Error: Unknown OpCode {:?}\n", instruction),
            }
        }

        // @todo Tmp value to return for testing
        // InterpretResult::Ok
        Ok(Value::Null)
    }
}