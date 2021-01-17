use super::error::ParsingError;
use super::parser_struct::Parser;

use crate::token::Token;
use crate::token_type::TokenType;

// Infrastructure/Utility methods on the Parser struct
impl Parser {
    // Returns immutable reference to the current token
    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    pub fn is_at_end(&self) -> bool {
        self.check(TokenType::Eof)
    }

    pub fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    pub fn advance(&mut self) -> &Token {
        // Old way of doing it.
        // Only increment the current token counter if not at end yet
        // if !self.is_at_end() {
        //     self.current += 1;
        // }
        // Get previous token without call to "previous" method to save the extra function call... but LLVM is probs smart enough to optimize this
        // self.tokens.get(self.current - 1).unwrap()

        // Assume caller will check if it is at the end of token vector so no need for extra check here
        self.current += 1;
        self.previous()
    }

    // Checks if current token matches the given type
    // If so, consumes the token and returns true.
    // Otherwise, returns false and leave current token alone
    pub fn is_next_token(&mut self, token_type_to_check: TokenType) -> bool {
        if self.check(token_type_to_check) {
            self.advance();
            true
        } else {
            false
        }
    }

    // Checks if current token matches any of the given types.
    // If so, consumes the token and returns true.
    // Otherwise, returns false and leave current token alone
    pub fn is_next_token_any_of_these(&mut self, token_types_to_check: Vec<TokenType>) -> bool {
        for token_type in token_types_to_check {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    // Checks and consumes token if it is of the specified type,
    // Else bubble up a UnexpectedToken ParsingError with the given string as its message
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParsingError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParsingError::UnexpectedTokenError(
                // @todo change parsing error to take ref instead?
                self.peek().clone(),
                message,
            ))
        }
    }
}
