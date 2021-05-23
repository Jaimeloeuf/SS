use super::parser::ParsingError;

use crate::opcode::OpCode;
use crate::token::TokenType;

// @todo Should this also encapsulate parsing error?
#[derive(Debug)]
pub enum CompileError {
    // Wrapper type over ParsingError to allow it to be bubbled up into CompileError eventually
    ParsingError(ParsingError),

    IdentifierAlreadyUsed(String),

    IdentifierNotInAnyLocalScope(String),

    /// Missing compiler/parser method in compiler struct to parse given expression
    MissingParser(String),

    /// 'return' can only be used in a function body, store line number with error for error message
    // ReturnOutsideFunction(Token),
    ReturnOutsideFunction(usize),

    /// If the number of arguments does not matched the number of parameters defined
    /// MismatchedArgumentCount(number_of_parameters, number_of_args),
    MismatchedArgumentCount(usize, usize),

    // Internal Compiler Errors
    // @todo Change these to be panics instead in the code directly, as these should not happen if compiler is bug free

    // If the opcode is not a JUMP type opcode when trying to back patch a JUMP opcode
    InvalidJumpOpcode(OpCode),

    /// If the operator used is not valid for the compiler method
    InvalidOperatorType(TokenType),
}

// Convert ParsingError to CompileError automatically
impl From<ParsingError> for CompileError {
    fn from(error: ParsingError) -> Self {
        CompileError::ParsingError(error)
    }
}
