use super::error::ParsingError;
use super::parser_struct::Parser;

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Stmt {
    //
}

impl Parser {
    // Consumes a token vector (takes ownership) to produce a statements vector (moved out)
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParsingError> {
        let mut parser = Parser { tokens, current: 0 };

        println!("Processing '{}' tokens", parser.tokens.len());

        let statements: Vec<Stmt> = Vec::<Stmt>::new();
        // let statements: Vec<Stmt> = Vec::new();

        // On each loop, we scan a single token.
        while !parser.is_at_end() {
            let expr = parser.expression();
            match expr {
                Ok(expr) => println!("Parsed expression: {}\n", expr),
                Err(e) => println!("Error parsing: {}\n", e),
            }

            parser.advance();
        }

        // Pass back immutable reference of the tokens vector wrapped in a Result variant
        Ok(statements)
    }

    // Synchronize the tokens to approx the next valid token
    pub fn synchronize(&mut self) {
        self.advance();

        // Loop till either EOF token or when one of the possible new start tokens is read
        // Where new start token, is a token that could indicate a new start where all previous syntax errors are behind it
        while !self.is_at_end() {
            // Stop synchronize loop when semicolon is read.
            // This assumes that in most cases, the error only cascades to a semicolon
            // This is a best case effort too, where it will fail when dealing with the semicolons in a for loop.
            // @todo Why is this previous? And cant this be in the match stmt?
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            // Matching the other TokenTypes
            // When these token types are read, stop the synchronize loop
            match self.peek().token_type {
                TokenType::Function
                | TokenType::Const
                | TokenType::If
                // | TokenType::For
                // | TokenType::Print
                | TokenType::While
                | TokenType::Return => return,
                _ => {}
            }

            // Advance to eat the current token
            self.advance();
        }
    }
}
