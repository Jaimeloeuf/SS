use crate::error::RuntimeError;

// @todo Deriving clone trait for now before we can have a way to move value out from OpCode without cloning
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Primitive types
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    // None primitive types, a.k.a wrapper for all complex user types
}

impl Value {
    // A faster alternative would be to just change the value in the stack directly
    // Rather then pop, negate, and push.
    // Abit hard to achieve above with rust it seems compared to C
    pub fn negate(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::Number(number) => Ok(Value::Number(-number)),

            _ => Err(RuntimeError::TypeError(format!("Only can negate numbers"))),
        }
    }

    // A faster alternative would be to just change the value in the stack directly
    // Rather then pop, not, and push.
    // Abit hard to achieve above with rust it seems compared to C
    pub fn not(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::Bool(bool) => Ok(Value::Bool(!bool)),

            _ => Err(RuntimeError::TypeError(format!("Only can 'not' bools"))),
        }
    }
}
