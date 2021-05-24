use crate::scanner::Scanner;
use crate::scanner::ScannerError;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser {
    pub scanner: Scanner,

    pub current: Token,
    pub previous: Token,
}

#[derive(Debug)]
pub enum ParsingError {
    ScannerError(ScannerError),

    /// @todo Change out the blanket error variant used temporarily for now
    error,
}

// Convert ScannerError to ParsingError automatically
impl From<ScannerError> for ParsingError {
    fn from(error: ScannerError) -> Self {
        ParsingError::ScannerError(error)
    }
}

impl Parser {
    pub fn new(scanner: Scanner, current: Token, previous: Token) -> Parser {
        Parser {
            current,
            previous,
            scanner,
        }
    }

    /// Utility method to make it easy to check current token's TokenType.
    /// No runtime method call overhead as it will be inlined.
    #[inline]
    pub fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    /// Checks if current token has the same TokenType as the method argument, if so advances parser and return true
    pub fn match_next(&mut self, token_type: TokenType) -> Result<bool, ParsingError> {
        Ok(if self.check(token_type) {
            self.advance()?;
            true
        } else {
            false
        })
    }

    /// Advance parser until the next token that is not TokenType::Error
    pub fn advance(&mut self) -> Result<(), ParsingError> {
        // Stash old current token as previous, to get the lexeme after matching a token.
        // Take the current token and place it into previous and create default placeholder token for self.current
        self.previous = std::mem::take(&mut self.current);

        // Keep reading tokens and reporting errors, until we hit a non-error or reach the end.
        // That way, the rest of the parser sees only valid tokens. The current and previous token are stored in the struct
        loop {
            // Steps forward through token stream, asking scanner for the next token and storing it
            self.current = self.scanner.scan_token()?;

            // Break once there isnt anymore error tokens
            if self.current.token_type != TokenType::Error {
                // Once there is a token that is not an error, we exit the loop to continue with whatever we are doing
                // The caller will access the tokens using previous and current on the parser struct
                return Ok(());
            } else {
                // Print the error out only, but dont stop.
                // self.error_at_current(self.current.start);
                // self.error_at_current("");

                // @todo Fix this error
                // Should this break out or?
                return Err(ParsingError::error);
            }
        }
    }

    // @todo Should bubble error up to be handled instead of internally here
    fn error_at(&self, token: &Token, message: String) {
        eprint!("[line {}] Error ", token.line);

        if token.token_type == TokenType::Eof {
            eprint!("at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            // Get slice from source and convert to String to print
            eprint!(
                "at '{}'",
                self.scanner.source[token.start..token.start + token.length].to_string()
            );
        }

        eprintln!(": {}", message);
        // parser.hadError = true;
    }

    // error_at for the current token.
    #[inline]
    fn error_at_current(&self, message: String) {
        self.error_at(&self.current, message)
    }

    /// Self needs to be parser
    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<(), ParsingError> {
        if self.current.token_type == token_type {
            self.advance()?;
        } else {
            self.error_at_current(message);
        }

        Ok(())
    }
}
