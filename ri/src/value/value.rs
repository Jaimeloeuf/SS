// Enum with all the possible variants of a Value object in SS as a dynamically typed language
use crate::callables::Callable;
use crate::interpreter::error::RuntimeError;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,

    // @todo Implement partial eq method
    Array(Vec<Value>),

    // Special Value variant that should only be used by return arm of interpret_stmt.
    // To indicate that this internal value should be bubbled up all the way to the nearest function block,
    // And then be used as the return value of that function call.
    Return(Box<Value>),

    // Why Rc<Callable> instead of Rc<Function>?
    // Because native functions simply impl Callable trait while, user functions are Function Structs that implement the Callable trait
    // We want to use a single interface for both function types, thus we use the common denominator between them, the Callable trait
    Func(Rc<dyn Callable>),
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
    // Strict boolean value check to get Boolean value from Value Type or return a RuntimeError of Value Type is not boolean
    // Allow caller to pass in String to used in runtime error
    pub fn bool_or_err(&self, error_string: &str) -> Result<bool, RuntimeError> {
        // Only match boolean value types, all else match to RuntimeError
        match *self {
            Value::Bool(b) => Ok(b),
            _ => Err(RuntimeError::TypeError(format!(
                "Expected Bool but found type and value: {:?}\n{}",
                self, error_string
            ))),
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
    // Takes line number of the token, which will be used by the RuntimeError if this fails
    pub fn callable(&self, line_number: usize) -> Result<Rc<dyn Callable>, RuntimeError> {
        // Only match callable value types, all else errors out
        match *self {
            // Why cant I borrow it out instead of clone?
            Value::Func(ref func) => Ok(Rc::clone(func)),
            // Value::Class(ref class) => Ok(Rc::clone(class)),
            _ => Err(RuntimeError::CallOnNonCallable(
                line_number,
                format!("{}", self), // Pass in String representation of Value using display trait to format it
            )),
        }
    }
}

// Essentially the pretty printer of values
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use ref to make sure the values are only borrowed and not moved
        match self {
            Value::Number(ref number) => write!(f, "{}", number),
            Value::String(ref string) => write!(f, "'{}'", string),
            Value::Bool(ref boolean) => write!(f, "{}", boolean),
            Value::Null => write!(f, "NULL"),

            // Use external function to print the array to use a loop to construct the final string
            // Potential problem with long vectors as the it will loop through all before returning the string, hogging memory and block CPU
            Value::Array(ref elements) => write!(f, "{}", print_array(elements)),

            Value::Return(ref value) => write!(f, "SS internal return value -> {}", value),

            Value::Func(ref func) => write!(f, "<function-{}>", func.to_string()),
        }
    }
}

// Returns string representation of an array of values
fn print_array(elements: &Vec<Value>) -> String {
    if elements.len() == 0 {
        // Special case to prevent 'elements.len() - 1' from panicking when length is 0
        String::from("[]")
    } else {
        let mut string = String::from("[");
        // Print out all but the last element in the array with a comma and space appended behind
        for element in elements.iter().take(elements.len() - 1) {
            string += &format!("{}, ", element);
        }
        // Add last element without any commas or space and add the closing bracket
        string + &format!("{}", elements.last().unwrap()) + &format!("]")
    }
}
