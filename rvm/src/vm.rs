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

// Macro to perform any generic binary operation on the last 2 values on the stack
// This macro should only used by the other binary operation macros
macro_rules! generic_binary_op {
    // $op_name     -> String literal name for the actual binary operation, used in error output for debugging
    // $stack       -> Takes in the identifier for the stack value too
    // $operator    -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    // $arm_pattern -> A match arm pattern for the types of values expected of the last 2 values on the stack
    // $arm_logic   -> An expression to execute and return if the last 2 values on the stack matched the arm_pattern
    ($op_name:literal, $stack:ident, $operator:tt, $arm_pattern:pat => $arm_logic:expr) => {
        // Pop the operands off the stack in reverse order, since the first operand will be loaded first
        // Loading will be, LOAD A, LOAD B, since stack is LIFO, when we pop out 2 values, it will be B then A
        let b = $stack.pop();
        let a = $stack.pop();

        // Only run this check during debug builds, assuming correctly generated OpCodes will not have this issue
        #[cfg(debug_assertions)]
        if a.is_none() || b.is_none(){
            panic!(format!("VM Debug Error: Stack missing values for {} operation '{}'",  $op_name, stringify!($operator)));
        }

        match (a, b) {
            $arm_pattern => $arm_logic,

            // If the last 2 values on the stack did not match the pattern described by $arm_pattern
            // The value types are assumed to be wrong, thus return Runtime TypeError
            (a, b) =>
                // Unwrap the values directly assuming that they are definitely Some() variants
                // If it fails, it means opcodes are generated wrongly where the stack is missing values needed for the opcode
                return Err(RuntimeError::TypeError(format!(
                    "Invalid operand types {:?} and {:?} used for '{}' {} operation",
                    a.unwrap(), b.unwrap(), stringify!($operator), $op_name
                )))
        }
    }
}

// Macro to perform a binary arithmetic operation (+, -, *, /) on the last 2 values on the stack
macro_rules! arithmetic_binary_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        generic_binary_op!(
            "Arithmetic",
            $stack,
            $operator,

            // Expect last 2 values on stack to be numbers, pushes a number back onto the stack
            (Some(Value::Number(num1)), Some(Value::Number(num2))) => $stack.push(Value::Number(num1 $operator num2))
        );
    }};
}

// Macro to perform a binary boolean equality operation (==, !=) on the last 2 values on the stack
macro_rules! equality_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        generic_binary_op!(
            "Equality",
            $stack,
            $operator,

            // Last 2 values on stack can be any Value enum variant, compares directly using Value's derived PartialEq trait and pushes a Bool back onto the stack
            (Some(value1), Some(value2)) => $stack.push(Value::Bool(value1 $operator value2))
        );
    }};
}

// Macro to perform a numeric comparison operation (>, >=, <, <=) on the last 2 values on the stack
macro_rules! numeric_comparison_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        generic_binary_op!(
            "Numeric Comparison",
            $stack,
            $operator,

            // Expect last 2 values on stack to be numbers, pushes a bool back onto the stack
            (Some(Value::Number(num1)), Some(Value::Number(num2))) => $stack.push(Value::Bool(num1 $operator num2))
        );
    }};
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

                OpCode::ADD => arithmetic_binary_op!(stack, +),
                OpCode::SUBTRACT => arithmetic_binary_op!(stack, -),
                OpCode::MULTIPLY => arithmetic_binary_op!(stack, *),
                OpCode::DIVIDE => arithmetic_binary_op!(stack, /),

                OpCode::NOT => {
                    let value = stack.pop().unwrap().not()?;
                    stack.push(value);
                }
                OpCode::NEGATE => {
                    let value = stack.pop().unwrap().negate()?;
                    stack.push(value);
                }

                OpCode::EQUAL => equality_op!(stack, ==),
                OpCode::NOT_EQUAL => equality_op!(stack, !=),
                OpCode::GREATER => numeric_comparison_op!(stack, >),
                OpCode::GREATER_EQUAL => numeric_comparison_op!(stack, >=),
                OpCode::LESS => numeric_comparison_op!(stack, <),
                OpCode::LESS_EQUAL => numeric_comparison_op!(stack, <=),

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
