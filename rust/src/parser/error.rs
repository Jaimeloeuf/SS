/**
 * Module for error handling of Parser errors.
 */
use crate::token::Token;
use std;

// @todo Specify lifetime for Tokens instead of taking ownership, which requires .clone() of token
#[derive(Debug)]
pub enum ParsingError {
    // Static string message are hardcoded compiler error messages
    UnexpectedTokenError(Token, &'static str),
    UnexpectedEofError(Token),
    InvalidAssignmentError(Token),
    TooManyArgumentsError,
    TooManyParametersError,
    // Static string message are hardcoded parser error messages
    InternalError(usize, &'static str),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedTokenError(ref token, message) => write!(
                f,
                "[line {}] Error: Unexpected Token Found -> {}\n\t{}",
                token.line, token, message,
            ),
            ParsingError::UnexpectedEofError(ref token) => {
                write!(f, "[line {}] Unexpected end of input", token.line)
            }
            ParsingError::InvalidAssignmentError(ref token) => {
                write!(f, "[line {}] Invalid assignment target", token.line)
            }
            ParsingError::InternalError(line, ref message) => {
                write!(f, "[line {}] Internal error: {}", line, message)
            }
            ParsingError::TooManyArgumentsError => {
                f.write_str("Too many arguments, max number is 8")
            }
            ParsingError::TooManyParametersError => {
                f.write_str("Too many parameters, max number is 8")
            }
        }
    }
}
