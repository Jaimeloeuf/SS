/*
    Scanner module to scan source file for a vector of tokens
*/

use super::error::ScannerError;
use super::scanner_struct::Scanner;

use crate::keywords::KEYWORDS;
use crate::token::Token;
use crate::token_type::TokenType;

impl Scanner {
    // Move ownership of source string into Scanner struct here
    pub fn scan_tokens(source: String) -> Vec<Token> {
        // Create new scanner struct to use internally
        let mut scanner = Scanner {
            source: source,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };

        // Each turn of the loop, we scan a single token.
        while !scanner.is_at_end() {
            // At the start of every loop, reset start of the current "line" to the current character's index
            scanner.start = scanner.current;

            // Scan source and add tokens if any to the "tokens" vector
            // Will get back either a token, no token (white spaces and comments) or an error
            match scanner.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {}
                Err(e) => errors.push(e),
            }
        }

        // Add Eof token
        scanner
            .tokens
            .push(Token::new_none_literal(TokenType::Eof, scanner.line));

        // Move token vector out of the scanner struct once scanning is completed
        // After calling scan_tokens, scanner is no longer used, thus is ok to transfer out ownership
        scanner.tokens
    }

    // Pass in a character to figure out what type of token it is
    // Will also eat and discard certain characters that are not used for the Token vector like newlines
    // Can be None, as some characters have no intrinsic token type, e.g. as white space
    fn get_token_type(
        &mut self,
        current_character: char,
    ) -> Result<Option<TokenType>, ScannerError> {
        // Match current_character (and maybe n next character(s)) to a TokenType or None
        // Minor optimization: Match arms are arranged in order of how frequently that character is expected
        match current_character {
            // Whitespace characters to be eaten and discarded
            // Because of how we parse, tabs should be preferred over spaces to reduce number of function calls to "get_token_type"
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),

            // Alphabetic words
            // Identifiers, must START with an alphabet or _, but can contain mix of alphanumeric chars
            'a'..='z' | 'A'..='Z' | '_' => Ok(Some(TokenType::Identifier)),

            // Newline characters causes line number to be incremented before being eaten and discarded
            '\n' => {
                self.line += 1;
                Ok(None)
            }

            ';' => Ok(Some(TokenType::Semicolon)),
            '{' => Ok(Some(TokenType::LeftBrace)),
            '}' => Ok(Some(TokenType::RightBrace)),
            '(' => Ok(Some(TokenType::LeftParen)),
            ')' => Ok(Some(TokenType::RightParen)),
            ',' => Ok(Some(TokenType::Comma)),
            '.' => Ok(Some(TokenType::Dot)),

            // Math operators
            '-' => Ok(Some(TokenType::Minus)),
            '+' => Ok(Some(TokenType::Plus)),
            '*' => Ok(Some(TokenType::Star)),

            // For lexeme that can be "chained" / have another char behind it to form a lexeme of 2 chars
            '!' if self.conditional_advance('=') => Ok(Some(TokenType::BangEqual)),
            '!' => Ok(Some(TokenType::Bang)),
            '=' if self.conditional_advance('=') => Ok(Some(TokenType::EqualEqual)),
            '=' => Ok(Some(TokenType::Equal)),
            '<' if self.conditional_advance('=') => Ok(Some(TokenType::LessEqual)),
            '<' => Ok(Some(TokenType::Less)),
            '>' if self.conditional_advance('=') => Ok(Some(TokenType::GreaterEqual)),
            '>' => Ok(Some(TokenType::Greater)),

            // |

            // Inline Comment, a comment that goes until the end of the line.
            '/' if self.conditional_advance('/') => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }

                /* Optimization:
                   Technically this is not needed, because if the next character is a new line,
                   It will be read, removed and have scanner struct's line incremented on the next call to "get_token_type"
                   The problem is that more often then not, the next char after this is usually a new line,
                   So instead of making another function call just to remove it,
                   we can do a much faster check right here and remove it if it exists.
                   Note this optimization can only be used for patterns that return None,
                   since we cannot increment line number before the caller of this function saves the token with the current line number
                */
                if self.peek() == '\n' {
                    self.advance();
                    self.line += 1;
                }

                Ok(None)
            }
            // Block Comment, comment that can span multiline lines
            '/' if self.conditional_advance('*') => {
                while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                    // Advance, AND if current char is a newline, increment line count
                    if self.advance() == '\n' {
                        self.line += 1;
                    }
                }

                // Advance 2 more times to eat the ending star and slash characters.
                self.advance();
                self.advance();

                /* Optimization:
                   Technically this is not needed, because if the next character is a new line,
                   It will be read, removed and have scanner struct's line incremented on the next call to "get_token_type"
                   The problem is that more often then not, the next char after this is usually a new line,
                   So instead of making another function call just to remove it,
                   we can do a much faster check right here and remove it if it exists.
                   Note this optimization can only be used for patterns that return None,
                   since we cannot increment line number before the caller of this function saves the token with the current line number
                */
                if self.peek() == '\n' {
                    self.advance();
                    self.line += 1;
                }

                Ok(None)
            }
            '/' => Ok(Some(TokenType::Slash)),

            // String Literals
            '"' => Ok(Some(TokenType::Str)),

            // Number Literals
            '0'..='9' => Ok(Some(TokenType::Number)),

            // Return Scanner Error if couldn't match any valid characters
            _ => Err(ScannerError {
                line: self.line,
                description: format!(
                    "Unexpected character '{}' on line {}",
                    current_character, self.line
                ),
            }),
        }
    }

    // Scan source and create Tokens before pushing them to the tokens vector
    fn scan_token(&mut self) {
        let current_character: char = self.advance();
        let token_type: Option<TokenType> = match self.get_token_type(current_character) {
            Ok(token_type) => token_type,
            Err(e) => {
                self.errors.push(e);
                None
            }
        };

        // Match TokenType to new Token, and handle processing needed for certain token types
        // Minor optimization: Match arms are arranged in order of how frequently that TokenType is expected
        match token_type {
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

            // Do nothing for None type TokenType
            None => (),

            Some(TokenType::Number) => {
                let literal = self.number_literal();
                self.tokens.push(Token::new_number(literal, self.line));
            }

            Some(TokenType::Str) => {
                let literal = self.string_literals();
                self.tokens.push(Token::new_string(literal, self.line));
            }

            // Last match arm to match all other unmatched token types that do not need special processing
            Some(token_type) => {
                // Can unwrap here as we are sure that there is a value because of the Some wrap matching
                self.tokens
                    .push(Token::new_none_literal(token_type, self.line));
            }
        };
    }

    // Returns the String literal between ""
    // A new string is created and returned instead of a slice as we do not want to move the characters out from self
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

    // Return string version of number literal to parse later during Token creation
    // A new string is created and returned instead of a slice as we do not want to move the characters out from self
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

        // Return as a new string first, then only convert it to its type later
        self.source[self.start..self.current].to_string()
    }

    // Get alphanumerical identifier string
    // A new string is created and returned instead of a slice as we do not want to move the characters out from self
    fn identifier(&mut self) -> String {
        // See link for the list of supported alphanumeric characters
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.source[self.start..self.current].to_string()
    }
}
