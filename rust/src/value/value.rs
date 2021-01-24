// Enum with all the possible variants of a Value object in SS as a dynamically typed language

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    // Func(Rc<Callable>),
    // Class(Rc<LoxClass>),
    // Instance(Rc<RefCell<LoxInstance>>),
}
