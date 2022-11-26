use super::error::ParsingError;
use super::parser_struct::Parser;
use super::stmt::Stmt;

use crate::token::Token;
use crate::token_type::TokenType;

/// Infrastructure/Utility methods on the Parser struct
impl Parser {
    /// Get a immutable reference to the current token without modifying the parser index.
    pub fn current(&self) -> &Token {
        // It is safe to unwrap here as before every call to this method, the parsing
        // loop will have already checked that it is not at the end of the vector yet.
        //
        // Although the alternative of returning an Option will be safer, it also
        // makes the calling code much more verbose.
        self.tokens.get(self.current_index).unwrap()
    }

    /// Check if the current token matches the given token type
    pub fn check(&self, token_type: TokenType) -> bool {
        self.current().token_type == token_type
    }

    /// Method to check if all of the tokens have been parsed.
    /// Currently this is done by checking if a EOF token is found instead of checking
    /// the length of the vector against the current index as it can be safely assumed
    /// that the EOF token is the last token in the vector.
    pub fn is_at_end(&self) -> bool {
        self.check(TokenType::Eof)
    }

    /// Get a immutable reference to the previous token without modifying the parser index.
    pub fn previous(&self) -> &Token {
        // It is safe to unwrap here as before every call to this method, the parsing
        // loop will have already checked that it is not at the end of the vector yet.
        //
        // Although the alternative of returning an Option will be safer, it also
        // makes the calling code much more verbose.
        self.tokens.get(self.current_index - 1).unwrap()
    }

    /// Simple utility method to advance the parser index.
    /// This is a inlined method used to make the usage of this more readable.
    /// Unlike the `get_current_token_and_advance` utility method, this does not return
    /// a reference to the token, use that method if the token reference is needed.
    #[inline]
    pub fn advance(&mut self) -> () {
        self.current_index += 1;
    }

    /// Get a immutable reference to the current token and increment the parser 'current_index'.
    pub fn get_current_token_and_advance(&mut self) -> &Token {
        // Old way of doing it.
        // Only increment the current token counter if not at end yet
        // if !self.is_at_end() {
        //     self.advance();
        // }
        // Get previous token without call to "previous" method to save the extra function call... but LLVM is probs smart enough to optimize this
        // self.tokens.get(self.current_index - 1).unwrap()

        // Assume caller will check if it is at the end of token vector so no need for extra check here.
        // Because when calling advance, the expected semantics is for the `current` method to advance
        // the current parser index and not conditionally advanced if not at end.
        //
        // Calling advance first so that a temporary variable is not required to hold the reference to
        // the current token before it is used as the last expression to be returned.
        self.advance();
        self.previous()
    }

    /// Checks if current token matches the given type.
    /// If matches, `consume` the token by advancing the parser index and returns true.
    /// Otherwise, returns false and leave current token alone
    pub fn is_next_token(&mut self, token_type_to_check: TokenType) -> bool {
        if self.check(token_type_to_check) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Checks if current token matches any of the given types.
    /// If matches, `consume` the token by advancing the parser index and returns true.
    /// Otherwise, returns false and leave current token alone
    pub fn is_next_token_any_of_these(&mut self, token_types_to_check: Vec<TokenType>) -> bool {
        for token_type in token_types_to_check {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Consume token if it is of the specified type, else bubble up a UnexpectedToken ParsingError
    /// with the given string as its message.
    /// Static string message to be passed in where the message is a hardcoded compiler error message
    pub fn consume(
        &mut self,
        token_type: TokenType,
        message: &'static str,
    ) -> Result<&Token, ParsingError> {
        if self.check(token_type) {
            Ok(self.get_current_token_and_advance())
        } else {
            Err(ParsingError::UnexpectedTokenError(
                // @todo change parsing error to take ref instead?
                self.current().clone(),
                message,
            ))
        }
    }

    /// Indirection for all declaration and statement methods, to call advance method first
    pub fn advance_and_call(
        &mut self,
        method: fn(&mut Parser) -> Result<Stmt, ParsingError>,
    ) -> Result<Stmt, ParsingError> {
        self.advance();
        method(self)
    }
}
