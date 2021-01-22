use super::error::ParsingError;
use super::expr::Expr;
use super::parser_struct::Parser;

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Stmt {
    //
}

impl Parser {
    // Consumes a token vector (takes ownership) to produce a statements vector (moved out)
    // @todo Tmp change of method API to return Vec of Expr instead of Stmt before that is implemented
    // pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParsingError> {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, Vec<ParsingError>> {
        let mut parser = Parser { tokens, current: 0 };

        println!("Processing '{}' tokens", parser.tokens.len());

        let mut statements: Vec<Expr> = Vec::<Expr>::new();
        let mut errors: Vec<ParsingError> = Vec::<ParsingError>::new();

        // On each loop, we scan a single token.
        while !parser.is_at_end() {
            // Get expression and based on output, push to either one of the vectors
            match parser.expression() {
                Ok(expr) => statements.push(expr),
                // For err, maybe I should log it to stderr at the same time too, so that LSP can pick it up?
                Err(e) => errors.push(e),
            }

            parser.advance();
        }

        // Return vector of statements only if there are no errors
        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn expression(&mut self) -> Result<Expr, ParsingError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.comparison()?;

        while self.is_next_token_any_of_these(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.term()?;

        while self.is_next_token_any_of_these(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.factor()?;

        while self.is_next_token_any_of_these(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.unary()?;

        while self.is_next_token_any_of_these(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParsingError> {
        if self.is_next_token_any_of_these(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Expr, ParsingError> {
        if self.is_next_token_any_of_these(vec![TokenType::True, TokenType::False, TokenType::Null])
        {
            Ok(Expr::Literal(self.previous().clone().token_type))
        } else if self.is_next_token_any_of_these(vec![
            TokenType::Identifier,
            TokenType::Str,
            TokenType::Number,
        ]) {
            Ok(Expr::Literal(self.previous().clone().token_type))
        } else if self.is_next_token(TokenType::LeftParen) {
            let expr = self.expression()?;

            // Check if there is a ")" to close the expression
            if let Err(e) = self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Err(e)
            } else {
                Ok(Expr::Grouping(Box::new(expr)))
            }
        } else {
            // I dont think we should use self.peek here
            Err(ParsingError::UnexpectedTokenError(
                (*self.peek()).clone(),
                "Invalid token found",
            ))
        }
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
