use super::Scanner;
use crate::keywords::get_token_type_if_keyword;

use crate::token::token::Token;
use crate::token::token_type::TokenType;

impl Scanner {
    // @todo Should be named compile function
    pub fn scan_tokens(source: String) {
        let mut scanner = Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        };

        // @todo To fix this later, using for printing to print 0 as number instead of |
        let mut line: isize = -1;

        loop {
            let token = scanner.scan_token();

            if token.line != line as usize {
                // Up to 9999 lines, after that printed values will not align
                print!("{:0width$} ", token.line, width = 4);
                line = token.line as isize;
            } else {
                print!("   | ");
            }

            println!("{:?} {} {}", token.token_type, token.start, token.length);

            // Exit the loop when Eof token is encountered
            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }

    fn scan_token(&mut self) -> Token {
        // Skips none essential characters like whitespaces and comments
        self.skip_none_essentials();

        self.start = self.current;

        if self.is_at_end() {
            self.make_token(TokenType::Eof)
        } else {
            match self.advance() {
                // Identifiers must START with an alphabet or _, but can contain mix of alphanumeric chars
                'a'..='z' | 'A'..='Z' | '_' => {
                    // See list of alphanumerics here https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
                    while self.peek().is_alphanumeric() {
                        self.current += 1;
                    }

                    // Get alphanumerical identifier string as a slice of self.source and test if it is a keyword
                    let identifier = &self.source[self.start..self.current];
                    let keyword_token_type = get_token_type_if_keyword(&identifier);

                    match keyword_token_type {
                        // If it is a keyword, we use that keyword's token type.
                        Some(token_type) => self.make_token(token_type),

                        // Otherwise, it's a regular user-defined identifier.
                        None => self.make_token(TokenType::Identifier),
                    }
                }

                ';' => self.make_token(TokenType::Semicolon),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '[' => self.make_token(TokenType::LeftBracket),
                ']' => self.make_token(TokenType::RightBracket),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                // Math operators
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '*' => self.make_token(TokenType::Star),
                '/' => self.make_token(TokenType::Slash),

                _ => {
                    // @todo Return a err variant of Result
                    self.make_token(TokenType::Error)
                }
            }
        }
    }
}
