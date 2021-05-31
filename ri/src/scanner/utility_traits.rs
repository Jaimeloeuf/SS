use super::scanner_struct::Scanner;

use crate::token::Token;
use crate::token_type::TokenType;

impl Scanner {
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // advance() is for input
    // Consume next character from source and return it.
    // Must be valid char else this will panic during the unwrap
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

    // Get next character in source string without advancing index of current character
    // Used to check lexical grammar
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    // Get next next character in source string without advancing index of current character
    // Used to check lexical grammar
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    // Simple wrapper around Token::new_none_literal to simplify none literal token creation
    // As alot of places reuses this syntax with just different token types, used as a inlined function
    #[inline]
    pub fn new_none_literal(&self, token_type: TokenType) -> Option<Token> {
        Some(Token::new_none_literal(token_type, self.line))
    }
}
