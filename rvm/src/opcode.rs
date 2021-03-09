use crate::value::Value;

// OpCodes variants can optionally contain additional values like 'CONSTANT(Value)' to be executed together with the code
// Clox differentiate OpCode from Data using OP_ prefix, and also all their data types is just a single byte anyways

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub enum OpCode {
    RETURN,
    CONSTANT(Value),

    // Unary operators
    NEGATE,
}
