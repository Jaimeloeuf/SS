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
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
    Bool,
    Null,

    /// Arrays expect homogenous data types
    Array(Box<Type>),

    /// Func(vector_of_parameter_types, return_type)
    Func(Vec<Box<Type>>, Box<Type>),

    /// Return is a special type that wraps a Type,
    /// The point of the Return type is to allow type checker to know and let it bubble up till a handler
    Return(Box<Type>),
}

// @todo Manually implement PartialEq for the higher level types like Functions and Arrays..?
