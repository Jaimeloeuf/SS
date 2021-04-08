use crate::value::Value;

// OpCodes variants can optionally contain additional values like 'CONSTANT(Value)' to be executed together with the code
// Clox differentiate OpCode from Data using OP_ prefix, and also all their data types is just a single byte anyways

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum OpCode {
    POP,
    POP_N(usize),
    RETURN,
    CONSTANT(Value),

    // Opcode to take and store last value on stack as a value with the given string identifier
    IDENTIFIER(String),
    // Opcode to take and value using given string identifier from hashmap and push it onto stack
    IDENTIFIER_LOOKUP(String),

    GET_LOCAL(usize),
    SET_LOCAL(usize),

    // JUMP type / control flow opcodes
    JUMP(usize),
    JUMP_IF_FALSE(usize),

    // Special opcodes to do type checking of the last value on the stack
    TYPE_CHECK_BOOL,

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
