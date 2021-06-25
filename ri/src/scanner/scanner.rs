/*
    Scanner module to scan source file for a vector of tokens
*/

use super::error::ScannerError;
use super::scanner_struct::Scanner;

use crate::keywords::get_token_type_if_keyword;
use crate::token::Token;
use crate::token_type::TokenType;

impl Scanner {
    // Move ownership of source string into Scanner struct here
    pub fn scan_tokens(source: String) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut errors: Vec<ScannerError> = Vec::<ScannerError>::new();

        // Create new scanner struct to use internally
        let mut scanner = Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        };

        // Scan token by token
        // @todo See if can avoid is_at_end() check as it is very inefficient
        while !scanner.is_at_end() {
            // At the start of every loop, reset start of the current "line" to the current character's index
            scanner.start = scanner.current;

            // Scan source and add tokens if any to the "tokens" vector
            // Will get back either a token, no token (white spaces and comments) or an error
            match scanner.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {}

                // @todo Question is should we continue to scan if there is an error?
                // yes right? For things like LSP, since we still want to be able to parse?
                // Do we need to synchronize too?
                Err(e) => errors.push(e),
            }
        }

        // Add Eof token
        tokens.push(Token::new_none_literal(TokenType::Eof, scanner.line));

        // Return token vector only if there are no errors
        if errors.is_empty() {
            Ok(tokens)
        } else {
            // Return errors if any and have the caller handle it
            // Might handle it differently depending on how many files are there for the program.
            Err(errors)
        }
    }

    // Match current character to new Token, and handle processing needed for certain token types that spans multiple chars like strings
    // Eats and discards characters for newlines and comments and returns None
    // Returns a ScannerError if the current character cannot be matched
    fn scan_token(&mut self) -> Result<Option<Token>, ScannerError> {
        let current_character: char = self.advance();

        // Wrap match expression in Ok variant instead of wrapping Token options with Ok variant in every arm
        // Err option inside match expression cannot evaluate and return implicitly due to the Ok wrapping,
        // thus it needs to be explicitly returned to break out of this Ok variant wrapping.
        //
        // Minor optimization: Match arms are arranged in order of how frequently that character type is expected
        Ok(match current_character {
            // Whitespace characters to be eaten and discarded
            // Because of how we parse, tabs are preferred over spaces to reduce number of times "scan_tokens" calls "scan_token"
            ' ' | '\r' | '\t' => None,

            // Identifiers, must START with an alphabet or _, but can contain mix of alphanumeric chars
            'a'..='z' | 'A'..='Z' | '_' => {
                // See link for the list of supported alphanumeric characters
                // https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
                while self.peek().is_alphanumeric() {
                    self.current += 1;
                }

                // Get alphanumerical identifier string as a slice of self.source and test if it is a keyword
                let identifier = &self.source[self.start..self.current];
                let keyword_token_type = get_token_type_if_keyword(&identifier);

                match keyword_token_type {
                    // If it is a keyword, we use that keyword's token type.
                    Some(token_type) => Some(Token::new_keyword(token_type, self.line)),

                    // Otherwise, it's a regular user-defined identifier.
                    None => Some(Token::new_identifier(identifier.to_string(), self.line)),
                }
            }

            // Newline characters causes line number to be incremented before being eaten and discarded
            '\n' => {
                self.line += 1;
                None
            }

            ';' => self.new_none_literal(TokenType::Semicolon),
            '{' => self.new_none_literal(TokenType::LeftBrace),
            '}' => self.new_none_literal(TokenType::RightBrace),
            '(' => self.new_none_literal(TokenType::LeftParen),
            ')' => self.new_none_literal(TokenType::RightParen),
            '[' => self.new_none_literal(TokenType::LeftBracket),
            ']' => self.new_none_literal(TokenType::RightBracket),
            ',' => self.new_none_literal(TokenType::Comma),
            '.' => self.new_none_literal(TokenType::Dot),

            // Math operators
            '-' => self.new_none_literal(TokenType::Minus),
            '+' => self.new_none_literal(TokenType::Plus),
            '*' => self.new_none_literal(TokenType::Star),

            // For lexeme that can be "chained" / have another char behind it to form a lexeme of 2 chars
            '!' if self.conditional_advance('=') => self.new_none_literal(TokenType::BangEqual),
            '!' => self.new_none_literal(TokenType::Bang),
            '=' if self.conditional_advance('>') => self.new_none_literal(TokenType::Arrow),
            '=' if self.conditional_advance('=') => self.new_none_literal(TokenType::EqualEqual),
            '=' => self.new_none_literal(TokenType::Equal),
            '<' if self.conditional_advance('=') => self.new_none_literal(TokenType::LessEqual),
            '<' => self.new_none_literal(TokenType::Less),
            '>' if self.conditional_advance('=') => self.new_none_literal(TokenType::GreaterEqual),
            '>' => self.new_none_literal(TokenType::Greater),

            // @todo Token types for binary operators?
            // |

            // Inline Comment, a comment that goes until the end of the line.
            '/' if self.conditional_advance('/') => {
                // @todo Need a faster way to do this as too slow now
                while self.peek() != '\n' && !self.is_at_end() {
                    self.current += 1;
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
                    self.current += 1;
                    self.line += 1;
                }

                None
            }

            // Block Comment, comment that can span multiline lines
            '/' if self.conditional_advance('*') => {
                // Loop till the end of comment block
                loop {
                    // Stop if the next 2 characters are '*/', OR if it is at EOF
                    if (self.peek() == '*' && self.peek_next() == '/') || self.is_at_end() {
                        break;
                    }

                    // Advance, AND if current char is a newline, increment line count
                    if self.advance() == '\n' {
                        self.line += 1;
                    }
                }

                // Alternative that is semantically equivalent using the while loop, which defines condition to continue,
                // instead of the condition to stop, which makes it harder to understand in this case.
                //
                // Keep scanning as long as next 2 characters are not '*/'
                // while (self.peek() != '*' || self.peek_next() != '/') && !self.is_at_end() {
                //     // Advance, AND if current char is a newline, increment line count
                //     if self.advance() == '\n' {
                //         self.line += 1;
                //     }
                // }

                // Advance current character pointer 2 more times to eat the ending star and slash characters.
                self.current += 1;
                self.current += 1;

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
                    self.current += 1;
                    self.line += 1;
                }

                None
            }

            '/' => self.new_none_literal(TokenType::Slash),

            // String Literals
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    // Allow multiline strings.
                    // @todo Is extra processing needed to remove the \n from the final string? Or keep as is?
                    if self.peek() == '\n' {
                        self.line += 1;
                    }

                    self.current += 1;
                }

                // @todo Return error variant instead
                if self.is_at_end() {
                    panic!("Unexpected Eof while parsing for string literal");
                }

                // Skip the closing double quote "
                self.current += 1;

                // Trim surrounding quotes and only use the actual string content for the token
                Some(Token::new_string(
                    self.source[self.start + 1..self.current - 1].to_string(),
                    self.line,
                ))
            }

            // Number Literals
            '0'..='9' => {
                // Keep consuming till none ascii
                while self.peek().is_ascii_digit() {
                    self.current += 1;
                }

                // Look for a fractional part "."
                if self.peek() == '.' && self.peek_next().is_ascii_digit() {
                    // Consume fractional notation "."
                    self.current += 1;

                    // Keep consuming till none ascii for the number behind the decimal point
                    while self.peek().is_ascii_digit() {
                        self.current += 1;
                    }
                }

                // Get &str slice from source before parsing it into a f64, the numerical type used
                // Unwrap directly as we assumed if scan correctly above, it can parse to a f64 no problem
                Some(Token::new_number(
                    self.source[self.start..self.current]
                        .parse::<f64>()
                        .unwrap(),
                    self.line,
                ))
            }

            // Return ScannerError if couldn't match any valid characters
            // Since the match statement is wrapped in Ok, we cannot let this evalute to Err variant, must return explicitly instead
            _ => {
                return Err(ScannerError {
                    line: self.line,
                    description: format!(
                        "Unexpected character '{}' on line {}",
                        current_character, self.line
                    ),
                });
            }
        })
    }
}
