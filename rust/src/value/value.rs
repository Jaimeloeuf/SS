// Enum with all the possible variants of a Value object in SS as a dynamically typed language
use crate::interpreter::error::RuntimeError;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    // Func(Rc<Callable>),
    // Class(Rc<LoxClass>),
    // Instance(Rc<RefCell<LoxInstance>>),
}

impl Value {
    // Strict boolean value check, checks both Value Type, and boolean value of Bool Type
    // Will return a RuntimeError if Value Type is not boolean
    pub fn bool(&self) -> Result<bool, RuntimeError> {
        // Only match boolean value types, all else evaluates to false
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(RuntimeError::TypeError(format!("Expected Bool!"))),
        }
    }
    // Strict boolean value check, checks both Value Type, and boolean value of Bool Type
    // Will return a RuntimeError if Value Type is not boolean
    // Allow caller to pass in String to use in runtime error
    pub fn bool_or_err(&self, error_string: &str) -> Result<bool, RuntimeError> {
        // Only match boolean value types, all else evaluates to false
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(RuntimeError::TypeError(format!(
                "Expected Bool! Invalid type and value: {}\n{}",
                self, error_string
            ))),
        }
    }
    // Strict boolean value check, checks both Value Type, and boolean value of Bool Type
    // Will return a RuntimeError if Value Type is not boolean
    // Allow caller to pass in a RuntimeError to be returned if bool check failed
    // @todo The problem with this is that the caller wont know what is the Value type since it is probably chained to interpret_expr()?
    pub fn bool_or(&self, err: RuntimeError) -> Result<bool, RuntimeError> {
        // Only match boolean value types, all else evaluates to false
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(err),
        }
    }

    // Boolean cast to test for truthy and falesy values
    // Not used right now as not sure if the language should support this
    // Only Bool(false) and Null evaluates to False while everything else evaluates to True
    // pub fn is_truthy(&self) -> bool {
    //     match *self {
    //         Value::Bool(b) => b,
    //         Value::Null => false,
    //         _ => true,
    //     }
    // }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use ref to make sure the values are only borrowed and not moved
        match self {
            Value::Number(ref number) => write!(f, "{}", number),
            Value::String(ref string) => write!(f, "'{}'", string),
            Value::Bool(ref boolean) => write!(f, "{}", boolean),
            Value::Null => write!(f, "NULL"),
        }
    }
}
