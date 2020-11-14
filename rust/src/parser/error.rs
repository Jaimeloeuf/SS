/**
 * Module for error handling of Parser errors.
 */
use crate::token::Token;
use std;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedTokenError(Token, String),
    UnexpectedEofError,
    InvalidAssignmentError(Token),
    TooManyArgumentsError,
    TooManyParametersError,
    InternalError(String),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParsingError::UnexpectedTokenError(ref token, ref message) => write!(
                f,
                "[line {}] UnexpectedTokenError: {}\n\nFound Token and Type: {} {}",
                token.line,
                message,
                match &token.literal {
                    Some(literal) => literal,
                    None => "",
                },
                token.token_type,
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
            ParsingError::UnexpectedTokenError(_, _) => "UnexpectedTokenError",
            ParsingError::UnexpectedEofError => "UnexpectedEofError",
            ParsingError::InvalidAssignmentError(_) => "InvalidAssignmentError",
            ParsingError::InternalError(_) => "InternalError",
            ParsingError::TooManyArgumentsError => "TooManyArgumentsError",
            ParsingError::TooManyParametersError => "TooManyParametersError",
        }
    }
}
