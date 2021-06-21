/**
 * Module for error handling of Parser errors.
 */
use crate::token::Token;
use std;

// @todo Specify lifetime for Tokens instead of taking ownership, which requires .clone() of token
#[derive(Debug)]
pub enum ParsingError {
    /// Static string message are hardcoded parser error messages
    UnexpectedTokenError(Token, &'static str),

    UnexpectedEofError(Token),

    /// EmptyBlockStatement(line_number_of_closing_right_brace)
    EmptyBlockStatement(usize),

    /// InternalError(line_number, error_message),
    ///
    /// Static string message are hardcoded parser error messages
    InternalError(usize, &'static str),
    /*
        Unused
    */
    // For if assignments are supported
    // InvalidAssignmentError(Token),
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
            ParsingError::InternalError(line, ref message) => {
                write!(f, "[line {}] Internal error: {}", line, message)
            }
            ParsingError::EmptyBlockStatement(line_number) => write!(
                f,
                "[line {}] Empty block statements are not allowed",
                line_number
            ),
            // ParsingError::InvalidAssignmentError(ref token) => {
            //     write!(f, "[line {}] Invalid assignment target", token.line)
            // }
        }
    }
}
