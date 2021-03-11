use crate::error::RuntimeError;

// @todo Deriving clone trait for now before we can have a way to move value out from OpCode without cloning
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
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
}