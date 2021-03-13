use super::Scanner;

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

                _ => {
                    // @todo Return a err variant of Result
                    self.make_token(TokenType::Error)
                }
            }
        }
    }
}
