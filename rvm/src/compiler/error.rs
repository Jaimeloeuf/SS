use crate::opcode::OpCode;
use crate::token::TokenType;

// @todo Should this also encapsulate parsing error?
#[derive(Debug)]
pub enum CompileError {
    IdentifierAlreadyUsed(String),
    MissingParser(String),
    InvalidOperatorType(TokenType),

    /* Internal Compiler Errors */
    // If the opcode is not a JUMP type opcode when trying to back patch a JUMP opcode
    InvalidJumpOpcode(OpCode),
}
