use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser {
    pub scanner: Scanner,

    pub current: Token,
    pub previous: Token,
}

enum ParsingError {
    error,
}

impl Parser {
    fn new(scanner: Scanner, current: Token, previous: Token) -> Parser {
        Parser {
            current,
            previous,
            scanner,
        }
    }

    pub fn advance(&mut self) -> Result<(), ParsingError> {
        // Stash old current token as previous, to get the lexeme after matching a token.
        self.previous = self.current;

        // Keep reading tokens and reporting errors, until we hit a non-error or reach the end.
        // That way, the rest of the parser sees only valid tokens. The current and previous token are stored in the struct
        loop {
            // Steps forward through token stream, asking scanner for the next token and storing it
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                // return Ok(());
                // Once there is a token that is not an error, we exit the loop to continue with whatever we are doing
                // The caller will access the tokens using previous and current on the struct
                break;
            } else {
                // Print the error out only, but dont stop.
                // self.error_at_current(self.current.start);
                // self.error_at_current("");
            }
        }

        return Ok(());
    }

    fn error_at(&self, token: &Token, message: String) {
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            // eprint!("{:0width$}", token.start, width = token.length);
            eprint!(" at '{}'", token.start);
        }

        eprintln!(": {}", message);
        // parser.hadError = true;
    }

    // error_at for the current token.
    #[inline]
    fn error_at_current(&self, message: String) {
        self.error_at(&self.current, message)
    }

    // Self needs to be parser
    pub fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }
}
