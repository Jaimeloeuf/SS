use super::Scanner;

use crate::token::Token;
use crate::token::TokenType;

impl Scanner {
    /// Method to create token using TokenType and the scanner's positional fields
    #[inline]
    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    /// Method checks if reached end of source code string
    #[inline]
    pub fn is_at_end(&self) -> bool {
        // Alternative way is to check if the current character is the terminating EOF
        // self.source.chars().nth(self.current).unwrap() == '\0'
        self.current >= self.source.len()
    }

    /// advance() is for input.
    /// Consume next character from source and return it.
    /// Must be valid char else this will panic during the unwrap.
    pub fn advance(&mut self) -> char {
        self.current += 1;
        // @todo What if the whole source is stored as a &str instead? Using source.as_str() then we just use the char with slices..? self.source[index..1]
        // https://stackoverflow.com/a/24542502/13137262
        // This is slow, as this access the char iteratively, but is neccessary as this allows UTF8 support in SS programs
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

    /// See what is the current character
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    /// See what is the next character
    /// For look ahead parsing
    pub fn peek_next(&self) -> char {
        // Return null terminator if reached end of source
        if self.is_at_end() {
            '\0'
        } else {
            // @todo Need another is at end check right? Because there is a plus 1...  -->  self.current + 1 >= self.source.len()
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    pub fn skip_none_essentials(&mut self) {
        // Return immediately if at end of source file
        if self.is_at_end() {
            return;
        }

        loop {
            match self.peek() {
                // Whitespace characters to be eaten and discarded
                // Because of how we parse, tabs are preferred over spaces to reduce number of loops/checks to discard them
                ' ' | '\r' | '\t' => self.current += 1,

                // Newline characters causes both line number and current character index to be incremented
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                }

                // Inline Comment, a comment that goes until the end of the line.
                '/' if self.peek_next() == '/' => {
                    // @todo Need a faster way to do this as too slow now. Perhaps inline the method calls?
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.current += 1;
                    }
                }

                // Block Comment, comment that can span multiline lines
                '/' if self.peek_next() == '*' => {
                    // Consume the '*' found in peek_next()
                    self.current += 1;

                    // Keep looping to advance current character index if,
                    // 1. The current and next character together is not '*/'
                    // 2. Eof not found and there are more characters
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        // Advance the current character index, AND if current char is a newline, increment line count
                        if self.advance() == '\n' {
                            self.line += 1;
                        }
                    }

                    // @todo What if at Eof? Will caller handle it with an additional check or?
                    // Advance current character pointer 2 more times to eat the ending star and slash characters.
                    self.current += 1;
                    self.current += 1;
                }

                // If character is not a none essential, then return control to caller to continue parsing it as a token
                _ => return,
            }
        }
    }
}
