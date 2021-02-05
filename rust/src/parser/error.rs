/**
 * Module for error handling of Parser errors.
 */
use crate::token::Token;
use std;

#[derive(Debug)]
pub enum ParsingError {
    // Static string message are hardcoded compiler error messages
    UnexpectedTokenError(Token, &'static str),
    UnexpectedEofError,
    InvalidAssignmentError(Token),
    TooManyArgumentsError,
    TooManyParametersError,
    // Static string message are hardcoded compiler error messages
    InternalError(&'static str),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedTokenError(token, message) => write!(
                f,
                "[line {}] UnexpectedTokenError: {}\nFound -> {}",
                token.line, message, token,
            ),
            ParsingError::UnexpectedEofError => f.write_str("Unexpected end of input"),
            ParsingError::InvalidAssignmentError(ref token) => {
                write!(f, "[line {}] Invalid assignment target", token.line)
            }
            ParsingError::InternalError(ref message) => write!(f, "Internal error: {}", message),
            ParsingError::TooManyArgumentsError => {
                f.write_str("Too many arguments, max number is 8")
            }
            ParsingError::TooManyParametersError => {
                f.write_str("Too many parameters, max number is 8")
            }
        }
    }
}

impl std::error::Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::UnexpectedTokenError(_, _) => "Unexpected Token",
            ParsingError::UnexpectedEofError => "Unexpected Eof",
            ParsingError::InvalidAssignmentError(_) => "Invalid Assignment",
            ParsingError::TooManyArgumentsError => "Too Many Arguments",
            ParsingError::TooManyParametersError => "Too Many Parameters",
            ParsingError::InternalError(_) => "Internal",
        }
    }
}
