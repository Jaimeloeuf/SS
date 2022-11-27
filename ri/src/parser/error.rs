//! Module for error handling of Parser errors.

use crate::token::Token;
use std::fmt;

// @todo Specify lifetime for Tokens instead of taking ownership, which requires .clone() of token
#[derive(Debug)]
pub enum ParsingError {
    /// UnexpectedTokenError takes static str compiler error messages as these are hardcoded
    /// in the parser and do not require any dynamic String message to be generated.
    UnexpectedTokenError(Token, &'static str),

    /// Only used if somehow there was an unexpected EOF token.
    UnexpectedEofError(Token),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::UnexpectedTokenError(ref token, message) => write!(
                f,
                "[line {}] Unexpected Token Found -> {}\n\t{}",
                token.line, token, message,
            ),
            ParsingError::UnexpectedEofError(ref token) => {
                write!(f, "[line {}] Unexpected end of input", token.line)
            }
        }
    }
}
