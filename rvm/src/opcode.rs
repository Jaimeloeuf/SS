use crate::value::Value;

// OpCodes variants can optionally contain additional values like 'CONSTANT(Value)' to be executed together with the code
// Clox differentiate OpCode from Data using OP_ prefix, and also all their data types is just a single byte anyways

#[derive(Debug)]
pub enum OpCode {
    POP,
    RETURN,
    CONSTANT(Value),

    // Opcode to take and store last value on stack as a value with the given string identifier
    IDENTIFIER(String),

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
