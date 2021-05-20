use crate::opcode::OpCode;
use crate::token::TokenType;

// @todo Should this also encapsulate parsing error?
#[derive(Debug)]
pub enum CompileError {
    IdentifierAlreadyUsed(String),

    /// Missing compiler/parser method in compiler struct to parse given expression
    MissingParser(String),

    InvalidOperatorType(TokenType),

    /* Internal Compiler Errors */
    // If the opcode is not a JUMP type opcode when trying to back patch a JUMP opcode
    InvalidJumpOpcode(OpCode),

    /// 'return' can only be used in a function body, store line number with error for error message
    ReturnOutsideFunction(usize),
}
