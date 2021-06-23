use std::collections::hash_map::HashMap;

use crate::parser::stmt::Stmt;
use crate::token::Token;

// Add lifetime specifier to String so that we can use ref of string instead of constantly cloning strings
pub struct TypeChecker {
    /// Using a vec as a "Stack" data structure
    ///
    /// @todo Might change this to a LinkedList
    pub scopes: Vec<HashMap<String, Type>>,

    /// Store the current function's identifier token in order to break out of recursive type checking
    ///
    /// Recursive type checking will occur because the type checker will see that it is a function call,
    /// and try to type check the function, even though the exact function is in the midst of being defined.
    ///
    /// Thus this field is used to defer type checking by comparing if TypeChecker is type checking the function definition,
    /// of the function that is being called, if the tokens are the same (fn name and line) it means that it is a recursive call,
    /// and thus the type checker should return Type::Lazy immediately as the type of the recursive function call,
    /// to make all checks against this recursive function call as valid, until it can actually be type checked with concrete types.
    pub current_function: Option<Token>,

    /// Field holding a vector of global identifiers
    ///
    /// Used by declare utility method to check if the identifier is a global identifier to give users a more specific error message
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

    /// Special type that is equal to all other types
    ///
    /// Used to defer type checking for types within a function definition, to when the types are available, such as during a function call
    Lazy,

    /// Arrays expect homogenous data types
    Array(Box<Type>),

    /// Func(number_of_parameters, function_stmt)
    ///
    /// The Function's AST node is stored so that it can be used to type check again when a function call is made
    Func(usize, Box<Stmt>),

    /// AnonymousFunc(number_of_parameters, function_stmt)
    ///
    /// This is just like the Func(..) variant except for the different name,
    /// which will be used to differentiate this and a named function during unused value check.
    AnonymousFunc(usize, Box<Stmt>),

    /// Return is a special type that wraps a Type,
    /// The point of the Return type is to allow type checker to know and let it bubble up till a handler
    Return(Box<Type>),
}

// Manually implement PartialEq trait for higher level types like Functions and Arrays that cannot be directly compared
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Every type is Equals to Lazy as the Lazy type is used to defer type checking till a later time
            (&Type::Lazy, _) | (_, &Type::Lazy) => true,

            // For primitive types, as long as they are the same, they are equal
            (&Type::Number, &Type::Number)
            | (&Type::String, &Type::String)
            | (&Type::Bool, &Type::Bool)
            | (&Type::Null, &Type::Null) => true,

            // For arrays, ensure that the types of their elements match
            (&Type::Array(ref array_element_type_1), &Type::Array(ref array_element_type_2)) => {
                array_element_type_1 == array_element_type_2
            }

            // For return types, ensure that the boxed types match
            (&Type::Return(ref return_type_1), &Type::Return(ref return_type_2)) => {
                return_type_1 == return_type_2
            }

            // (&Type::Func(ref f), &Type::Func(ref other)) => Rc::ptr_eq(f, other),
            // (&Type::Class(ref c), &Type::Class(ref other)) => Rc::ptr_eq(c, other),
            // (&Type::Instance(ref i), &Type::Instance(ref other)) => Rc::ptr_eq(i, other),
            _ => false,
        }
    }
}
