use super::scanner_struct::Scanner;

use crate::token::Token;
use crate::token_type::TokenType;

/// Implementation of all the utility traits.
/// Seperated from the main trait implementations to make it more readable.
impl Scanner {
    /// Check if scanner has reached the end of the source file string.
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advance the scanner by consuming the next character from source and returning it.
    /// Must be a valid character else this will panic during the unwrap.
    pub fn advance(&mut self) -> char {
        self.current += 1;
        // @todo Optimize this
        // This is slow, as this gets char iteratively instead of doing a direct array access/lookup
        // But is neccessary as this allows UTF8 support in SS programs
        // Reference: https://stackoverflow.com/a/24542502/13137262
        self.source.chars().nth(self.current - 1).unwrap()

        // If needed, push current character back into source as advance methods removes it.
        // self.source.push(current_character);
    }

    /// This is a conditional `advance()`, as it only consumes the current character if
    /// it matches what is passed in.
    pub fn conditional_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source.chars().nth(self.current).unwrap() != expected) {
            false
        } else {
            // Advance if the expected character is found
            self.current += 1;
            true
        }
    }

    /// Peek is used to check lexical grammar while scanning, by getting the next character
    /// in source string without advancing the current character index.
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    /// Peek next is used to check lexical grammar while scanning, by getting the next next
    /// character in source string without advancing the current character index.
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    /// Simple wrapper around Token::new_none_literal to simplify none literal token creation.
    /// This is a inlined method as alot of places reuse this syntax only with different token types.
    #[inline]
    pub fn new_none_literal(&self, token_type: TokenType) -> Option<Token> {
        Some(Token::new_none_literal(token_type, self.line))
    }
}
