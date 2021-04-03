use crate::token::TokenType;

// @todo Should this also encapsulate parsing error?
#[derive(Debug)]
pub enum CompileError {
    IdentifierAlreadyUsed(String),
    MissingParser(String),
    InvalidOperatorType(TokenType),
}
