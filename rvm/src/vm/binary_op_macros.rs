// Macro to perform any generic binary operation on the last 2 values on the stack
// This macro should only used by the other binary operation macros
#[macro_export]
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
            panic!("VM Debug Error: Stack missing values for {} operation '{}'",  $op_name, stringify!($operator));
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
#[macro_export]
macro_rules! arithmetic_binary_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        // @todo For whatever reason, only works if I macro_export generic macro and use it here with crate:: prefix
        crate::generic_binary_op!(
            "Arithmetic",
            $stack,
            $operator,

            // Expect last 2 values on stack to be numbers, pushes a number back onto the stack
            (Some(Value::Number(num1)), Some(Value::Number(num2))) => $stack.push(Value::Number(num1 $operator num2))
        );
    }};
}

// Macro to perform a binary boolean equality operation (==, !=) on the last 2 values on the stack
#[macro_export]
macro_rules! equality_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        // @todo For whatever reason, only works if I macro_export generic macro and use it here with crate:: prefix
        crate::generic_binary_op!(
            "Equality",
            $stack,
            $operator,

            // Last 2 values on stack can be any Value enum variant, compares directly using Value's derived PartialEq trait and pushes a Bool back onto the stack
            (Some(value1), Some(value2)) => $stack.push(Value::Bool(value1 $operator value2))
        );
    }};
}

// Macro to perform a numeric comparison operation (>, >=, <, <=) on the last 2 values on the stack
#[macro_export]
macro_rules! numeric_comparison_op {
    // $stack ->  Takes in the identifier for the stack value too
    // $operator -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    ($stack:ident, $operator:tt) => {{
        // @todo For whatever reason, only works if I macro_export generic macro and use it here with crate:: prefix
        crate::generic_binary_op!(
            "Numeric Comparison",
            $stack,
            $operator,

            // Expect last 2 values on stack to be numbers, pushes a bool back onto the stack
            (Some(Value::Number(num1)), Some(Value::Number(num2))) => $stack.push(Value::Bool(num1 $operator num2))
        );
    }};
}
