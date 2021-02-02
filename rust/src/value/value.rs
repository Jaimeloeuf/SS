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
