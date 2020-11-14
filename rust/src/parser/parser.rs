use super::parser_struct::Parser;

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Stmt {
    //
}

impl Parser {
    // Constructor
    // Takes ownership of the token vector
    pub fn new(tokens: Vec<Token>) -> Parser {
        println!("Processing '{}' tokens", tokens.len());
        Parser { tokens, current: 0 }
    }

    // Statements and State parse-declaration statements.add(statement());
    // Moves a statments vector out. Move instead of borrow as vec created in this scope
    pub fn parse(&mut self) -> Vec<Stmt> {
        let statements: Vec<Stmt> = Vec::<Stmt>::new();
        // let statements: Vec<Stmt> = Vec::new();

        // On each loop, we scan a single token.
        while !self.is_at_end() {
            self.advance();
        }

        // Pass back immutable reference of the tokens vector
        statements
    }
}
