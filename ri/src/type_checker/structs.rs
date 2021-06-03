use std::collections::hash_map::HashMap;

// Add lifetime specifier to String so that we can use ref of string instead of constantly cloning strings
pub struct TypeChecker {
    // Using a vec as a "Stack" data structure
    // @todo Might change this to a LinkedList
    pub scopes: Vec<HashMap<String, Type>>,

    // Tracker to see if currently in a function or not
    // Used to see if return statements are valid
    pub in_function: bool,

    // Field holding a vector of global identifiers
    // Used by declare utility method to check if the identifier is a global identifier to give users a more specific error message
    pub globals: Vec<&'static str>,
}

/// An enum of all possible types of values in SS
///
/// Need clone trait tmp in TypeChecker's symbol table
#[derive(Debug, Clone)]
pub enum Type {
    Number,
    String,
    Bool,
    Null,

    Lazy,

    /// Arrays expect homogenous data types
    Array(Box<Type>),

    /// Func(number_of_parameters, return_type)
    Func(usize, Box<Type>),

    /// Return is a special type that wraps a Type,
    /// The point of the Return type is to allow type checker to know and let it bubble up till a handler
    Return(Box<Type>),
}

// Manually implement PartialEq trait for higher level types like Functions and Arrays that cannot be directly compared
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Every type is Equals to Lazy, since the Lazy type is just a place holder
            (&Type::Lazy, _) | (_, &Type::Lazy) => true,

            // For primitive types, as long as they are the same, they are equal
            (&Type::Number, &Type::Number)
            | (&Type::String, &Type::String)
            | (&Type::Bool, &Type::Bool)
            | (&Type::Null, &Type::Null) => true,

            // For arrays, ensure that the types of their elements match
            (&Type::Array(ref array_element_type_1), &Type::Array(ref array_element_type_2)) => {
                // Not too sure if these work, since the ele type are Boxed values
                array_element_type_1 == array_element_type_2
            }

            // (&Type::Func(ref f), &Type::Func(ref other)) => Rc::ptr_eq(f, other),
            // (&Type::Class(ref c), &Type::Class(ref other)) => Rc::ptr_eq(c, other),
            // (&Type::Instance(ref i), &Type::Instance(ref other)) => Rc::ptr_eq(i, other),
            _ => false,
        }
    }
}
