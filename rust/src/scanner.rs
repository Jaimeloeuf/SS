/*
    Scanner module to scan source file for a vector of tokens
*/

use crate::keywords::KEYWORDS;
use crate::token::Token;
use crate::token_type::TokenType;

// All integer fields are limited by the size of an unsigned integer of the target system
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    // usize for fn is_at_end -> bool cos the source.len is of type usize
    start: usize, // start field points to the first character in the lexeme being scanned
    current: usize, // current points at the character currently being considered

    // This tracks the line scanner is currently on in the source file to produce tokens that know their location and for error reporting
    line: usize,
}

impl Scanner {
    // Constructor
    // Move ownership of source string into Scanner struct here
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // Expects self to be moved in instead of being a reference to move out a value from Scanner struct once done.
    pub fn scan_tokens(mut self) -> Vec<Token> {
        // Each turn of the loop, we scan a single token.
        while !self.is_at_end() {
            // At the start of every loop, reset start of the current "line" to the current character's index
            self.start = self.current;

            // Scan source and create Token struct as needed and add it to "tokens" vector
            self.scan_token();
        }

        // Add Eof token
        self.tokens
            .push(Token::new_none_literal(TokenType::Eof, self.line));

        // Move token vector out of the scanner struct once scanning is completed
        // After calling scan_tokens, scanner is no longer used, thus is ok to transfer out ownership
        self.tokens
    }

    // Pass in a character to figure out what type of token it is
    // Can be None, as some characters have no intrinsic token type, e.g. as white space
    fn get_token_type(&mut self, current_character: char) -> Option<TokenType> {
        // Match current_character (and maybe n next character(s)) to a TokenType or None
        match current_character {
            ';' => Some(TokenType::Semicolon),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),

            // Math operators
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '*' => Some(TokenType::Star),

            // For lexeme that can be "chained" / have another char behind it to form a lexeme of 2 chars
            '!' if self.conditional_advance('=') => Some(TokenType::BangEqual),
            '!' => Some(TokenType::Bang),
            '=' if self.conditional_advance('=') => Some(TokenType::EqualEqual),
            '=' => Some(TokenType::Equal),
            '<' if self.conditional_advance('=') => Some(TokenType::LessEqual),
            '<' => Some(TokenType::Less),
            '>' if self.conditional_advance('=') => Some(TokenType::GreaterEqual),
            '>' => Some(TokenType::Greater),

            // |

            // Inline Comment, a comment that goes until the end of the line.
            '/' if self.conditional_advance('/') => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
            }
            // Block Comment, comment that can span multiline lines
            '/' if self.conditional_advance('*') => {
                while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                    self.advance();
                }
                None
            }
            '/' => Some(TokenType::Slash),

            // Whitespace input types
            ' ' => None,
            '\r' => None,
            '\t' => None,

            '\n' => {
                self.line += 1;
                None
            }

            // String Literals
            '"' => Some(TokenType::Str),

            // Number Literals
            '0'..='9' => Some(TokenType::Number),

            // Alphabetic words
            // Identifiers, must start with alphabet or _, but can contain mix of alphanumeric chars
            'a'..='z' | 'A'..='Z' | '_' => Some(TokenType::Identifier),

            // Couldn't Match
            _ => {
                println!(
                    "Unexpected character '{}' on line {}",
                    current_character, self.line
                );
                // @todo Call the error handling code

                None
            }
        }
    }

    // Scan source and create Tokens before pushing them to the tokens vector
    fn scan_token(&mut self) {
        let current_character: char = self.advance();
        let token_type: Option<TokenType> = self.get_token_type(current_character);

        // Match TokenType to new Token, and handle processing needed for certain token types
        match token_type {
            Some(TokenType::Str) => {
                let literal = self.string_literals();
                self.tokens.push(Token::new_string(literal, self.line));
            }

            Some(TokenType::Number) => {
                let literal = self.number_literal();
                self.tokens.push(Token::new_number(literal, self.line));
            }

            Some(TokenType::Identifier) => {
                let identifier = self.identifier();
                let keyword_token_type = KEYWORDS.get(&identifier);

                match keyword_token_type {
                    // If it is a keyword, we use that keyword's token type.
                    Some(keyword) => self
                        .tokens
                        // @todo How to force move here instead of clone
                        .push(Token::new_keyword(keyword.clone(), self.line)),

                    // Otherwise, it's a regular user-defined identifier.
                    None => self
                        .tokens
                        .push(Token::new_identifier(identifier, self.line)),
                };
            }

            Some(token_type) => {
                // Can unwrap here as we are sure that there is a value because of the Some wrap matching
                self.tokens
                    .push(Token::new_none_literal(token_type, self.line));
            }

            // Do nothing for None type TokenType
            None => (),
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // advance() is for input
    // Consume next character from source and return it.
    // Must be valid char else this will panic during the unwrap
    // Push current character back into source as advance methods removes it.
    // @todo Or maybe advance method shouldnt remove it? And just borrow ref here?
    // self.source.push(current_character);
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    // This is a conditional advance(). Only consumes current character if it's what we're looking for.
    fn conditional_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source.chars().nth(self.current).unwrap() != expected) {
            return false;
        }

        // Advance if the expected character is found
        self.current += 1;

        true
    }

    // Get next character in source string without advancing index of current character
    // Used to check lexical grammar
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    // Get next next character in source string without advancing index of current character
    // Used to check lexical grammar
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    // Returns the String literal between ""
    // @todo Should this be a new string or a slice?
    fn string_literals(&mut self) -> String {
        while self.peek() != '"' && !self.is_at_end() {
            // Allow multiline strings.
            // @todo Is extra processing needed to remove the \n from the final string? Or keep as is?
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        // @todo Throw error here, something like SS.error(line, "Unterminated string.");
        if self.is_at_end() {
            println!("Unterminated string.");
            return "".to_string(); // @todo Fix this... I need this to return smth, but shouldnt cos this should just error out
        }

        // The closing double quote "
        self.advance();

        // Trim surrounding quotes and only return the actual string content
        self.source[self.start + 1..self.current - 1].to_string()
    }

    // @todo Should this be a new string or a slice?
    // fn number_literal(&mut self) -> isize {
    // Return string version of number literal to parse later as cannot determine type right now
    fn number_literal(&mut self) -> String {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part "."
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume fractional notation "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // This should not be isize, as the value will be limited.
        // @todo This will fail if it is a fraction
        // self.source[self.start..self.current]
        //     .parse::<isize>()
        //     .unwrap()
        // Return as a string first, then only convert it to its type later
        self.source[self.start..self.current].to_string()
    }

    fn identifier(&mut self) -> String {
        // See link for the list of supported alphanumeric characters
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.source[self.start..self.current].to_string()
    }
}
