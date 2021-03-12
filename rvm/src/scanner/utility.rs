use super::Scanner;

use crate::token::token::Token;
use crate::token::token_type::TokenType;

impl Scanner {
    // Method to create token using TokenType and the scanner's positional fields
    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    // Method checks if reached end of source code string
    pub fn is_at_end(&self) -> bool {
        // Alternative way is to check if the current character is the terminating EOF
        // self.source.chars().nth(self.current).unwrap() == '\0'
        self.current >= self.source.len()
    }

    // advance() is for input
    // Consume next character from source and return it.
    // Must be valid char else this will panic during the unwrap
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    // This is a conditional advance(). Only consumes current character if it's what we're looking for.
    pub fn conditional_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source.chars().nth(self.current).unwrap() != expected) {
            false
        } else {
            // Advance if the expected character is found
            self.current += 1;
            true
        }
    }
}
