// Enum with all the possible variants of a Value object in SS as a dynamically typed language

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
    pub fn is_bool_true(&self) -> bool {
        // Only match boolean value types, all else evaluates to false
        match *self {
            Value::Bool(b) => b,
            _ => false,
        }
    }

    // Boolean cast to test for truthy and falesy values
    // Not used right now as not sure if the language should support this
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
            Value::String(ref string) => write!(f, "{}", string),
            Value::Bool(ref boolean) => write!(f, "{}", boolean),
            Value::Null => write!(f, "Null"),
        }
    }
}
