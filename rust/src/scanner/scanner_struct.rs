use super::error::ScannerError;
use crate::token::Token;

// All integer fields are limited by the size of an unsigned integer of the target system
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<ScannerError>,

    // usize for fn is_at_end -> bool cos the source.len is of type usize
    pub start: usize, // start field points to the first character in the lexeme being scanned
    pub current: usize, // current points at the character currently being considered

    // This tracks the line scanner is currently on in the source file to produce tokens that know their location and for error reporting
    pub line: usize,
}
