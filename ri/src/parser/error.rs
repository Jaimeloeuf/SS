//! Module for error handling of Parser errors.

use crate::token::Token;
use std;

// @todo Specify lifetime for Tokens instead of taking ownership, which requires .clone() of token
#[derive(Debug)]
pub enum ParsingError {
    /// Static string message are hardcoded parser error messages
    UnexpectedTokenError(Token, &'static str),

    UnexpectedEofError(Token),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
