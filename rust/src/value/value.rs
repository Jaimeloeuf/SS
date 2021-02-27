// Enum with all the possible variants of a Value object in SS as a dynamically typed language
use crate::callables::Callable;
use crate::interpreter::error::RuntimeError;

use std::rc::Rc;

// #[derive(Debug, PartialEq, Clone)]
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,

    // Why Rc<Callable> instead of Rc<Function>?
    // Because native functions simply impl Callable trait while, user functions are Function Structs that implement the Callable trait
    // We want to use a single interface for both function types, thus we use the common denominator between them, the Callable trait
    Func(Rc<Callable>),
    // Class(Rc<LoxClass>),
    // Instance(Rc<RefCell<LoxInstance>>),
}

// PartialEq implementation for Value because Value cannot simply inherit this trait because of complex types like Rc<Callable>
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Value::Number(number), &Value::Number(other)) => number == other,
            (&Value::String(ref string), &Value::String(ref other)) => string == other,
            (&Value::Bool(b), &Value::Bool(other)) => b == other,
            (&Value::Null, &Value::Null) => true,
            (&Value::Func(ref f), &Value::Func(ref other)) => Rc::ptr_eq(f, other),
            // (&Value::Class(ref c), &Value::Class(ref other)) => Rc::ptr_eq(c, other),
            // (&Value::Instance(ref i), &Value::Instance(ref other)) => Rc::ptr_eq(i, other),
            _ => false,
        }
    }
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
                "Expected Bool but found type and value: {:?}\n{}",
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

    // Method to get callable if Value is a Callable value type, else errors out
    pub fn callable(&self) -> Result<Rc<Callable>, RuntimeError> {
        // Only match callable value types, all else errors out
        match *self {
            // Why cant I borrow it out instead of clone?
            Value::Func(ref func) => Ok(Rc::clone(func)),
            // Value::Class(ref class) => Ok(Rc::clone(class)),

            // @todo How to get the token?
            // _ => Err(RuntimeError::CallOnNonCallable(token)),
            _ => Err(RuntimeError::InternalError(format!("Non callable"))),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use ref to make sure the values are only borrowed and not moved
        match self {
            Value::Number(ref number) => write!(f, "{}", number),
            Value::String(ref string) => write!(f, "'{}'", string),
            Value::Bool(ref boolean) => write!(f, "{}", boolean),
            Value::Null => write!(f, "NULL"),

            Value::Func(ref func) => write!(f, "function-{}", func.to_string()),
        }
    }
}
