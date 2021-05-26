use crate::value::Value;

// OpCodes variants can optionally contain additional values like 'CONSTANT(Value)' to be executed together with the code
// Clox differentiate OpCode from Data using OP_ prefix, and also all their data types is just a single byte anyways

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum OpCode {
    /// POP a single value off the stack
    POP,
    /// POP 'usize' number of values off the stack
    POP_N(usize),
    /// Return from current function body. A.k.a go back once in the call stack
    RETURN,
    /// Return from current function body. A.k.a go back once in the call stack, and POP locals off the stack
    RETURN_POP(usize),

    /* Opcodes dealing with values/identifiers/variables */
    /// Load a Value onto the stack
    CONSTANT(Value),
    /// Take and store last value on stack as a value with the given string identifier
    IDENTIFIER(String),
    /// Get value using given string identifier from 'global scope storage' (hashmap) and push it onto stack
    IDENTIFIER_LOOKUP(String),
    /// GET a local scope value, by cloning the stack value at index 'usize' and pushing it onto stack
    GET_LOCAL(usize),
    /// Update an identifier in local scope, by setting the stack value at index 'usize' to the last value on stack
    SET_LOCAL(usize),

    /* JUMP type / control flow opcodes */
    /// JUMP forward by 'usize' number of opcodes. See OpCode::LOOP(usize) for jumping backwards
    JUMP(usize),
    JUMP_IF_FALSE(usize),

    /* Function call opcodes */
    /// CALL('number of arguements on stack')
    ///
    /// Stack: ... --> Value::Function(..) --> arg1 --> arg2 --> argN
    ///
    /// Make a function call, where the 'stack.len() - 1 - number_of_args' value on stack is the function 'Value::Fn(..)'
    CALL(usize),

    /// Special loop opcode, that is basically JUMP, but jumps backwards instead of forward
    LOOP(usize),

    /* Special opcodes to do type checking of the last value on the stack */
    /// Check if last value on stack is a Boolean
    TYPE_CHECK_BOOL,
    TYPE_CHECK(Value),

    // Arithmetic Binary operators
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,

    // Unary operators
    NOT,
    NEGATE,

    // Equality and Comparison operators
    EQUAL,
    NOT_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    PRINT,
}
