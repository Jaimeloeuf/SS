use super::Type;
use crate::token::Token;

#[derive(Debug)]
pub enum TypeError {
    /// @todo Conver to panic!
    InternalError(&'static str),

    /// Mismatched types error
    TypeError(String),

    /// Unused values:
    /// - Unused function call expressions that evaluates to a value
    /// - Literal values that are not used/binded to anything
    /// - Anonymous functions that are not used directly e.g. like in a return statement or binded to a identifier
    UnusedValue(Type),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::InternalError(ref message) => write!(f, "{}", message),

            TypeError::TypeError(ref message) => write!(f, "{}", message),

            TypeError::UnusedValue(type_of_unused_value) => write!(
                f,
                "All values must be used, found unused value of type: {:?}",
                type_of_unused_value
            ),
        }
    }
}
