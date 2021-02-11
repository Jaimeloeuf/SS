use super::error::ParsingError;
use super::expr::Expr;
use super::parser_struct::Parser;
use super::stmt::Stmt;
use crate::literal::Literal;

use crate::token::Token;
use crate::token_type::TokenType;

impl Parser {
    // Consumes a token vector (takes ownership) to produce a statements vector (moved out)
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Vec<ParsingError>> {
        let mut parser = Parser {
            tokens,
            currentIndex: 0,
        };

        println!("Processing '{}' tokens", parser.tokens.len());

        let mut statements: Vec<Stmt> = Vec::<Stmt>::new();
        let mut errors: Vec<ParsingError> = Vec::<ParsingError>::new();

        // On each loop, we scan a single token.
        while !parser.is_at_end() {
            // Get statement and based on output, push to either one of the vectors
            match parser.statement() {
                Ok(stmt) => statements.push(stmt),
                // Ok(stmt) => {
                //     println!("parsed stmt/expr {:?}", stmt);
                //     statements.push(stmt);
                // }

                // @todo Maybe log err to stderr too for the LSP to pick it up?
                // Add error to error vector, and synchronize the parser to continue parsing
                Err(e) => {
                    errors.push(e);
                    parser.synchronize();
                }
            }
        }

        // Return vector of statements only if there are no errors
        if errors.is_empty() {
            Ok(statements)
        } else {
            // Return errors if any and have the caller handle it
            // Might handle it differently depending on how many files are there for the program.
            Err(errors)
        }
    }

    /* ==========================  Start of statement methods  ========================== */

    fn statement(&mut self) -> Result<Stmt, ParsingError> {
        // Call the different statement parsing methods base on token_type
        // Else if not a statement token, it must be a expression statement
        match &self.current().token_type {
            TokenType::Print => self.call_stmt_method(Parser::print_statement),
            // TokenType::LeftBrace => self.call_stmt_method(Parser::leftbrace_statement()),
            // TokenType::If => self.call_stmt_method(Parser::if_statement()),
            // TokenType::While => self.call_stmt_method(Parser::while_statement()),
            // TokenType::For => self.call_stmt_method(Parser::for_statement()),
            // TokenType::Return => self.call_stmt_method(Parser::return_statement()),
            _ => self.expression_statement(),
        }
    }

    // Indirection for all statement methods, to call advance method first
    fn call_stmt_method(
        &mut self,
        method: fn(&mut Parser) -> Result<Stmt, ParsingError>,
    ) -> Result<Stmt, ParsingError> {
        self.advance();
        method(self)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        // Pass in the expression too to make it easier for user to fix the issue
        // self.consume(
        //     TokenType::Semicolon,
        //     format!("Expect ';' after value {}", expr),
        // )?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }

    /* ==========================  End of statement methods  ========================== */

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
        // Check for Literal values True/False/Null first before moving on to Identifier/Strings/Numbers and lastly grouped expressions
        // @todo Boolean types can we still be represented using TokenType, so should literal Bool values be used?
        if self.is_next_token(TokenType::True) {
            Ok(Expr::Literal(Literal::Bool(true)))
        } else if self.is_next_token(TokenType::False) {
            Ok(Expr::Literal(Literal::Bool(false)))
        } else if self.is_next_token(TokenType::Null) {
            Ok(Expr::Literal(Literal::Null))
        } else if self.is_next_token_any_of_these(vec![
            TokenType::Identifier,
            TokenType::Str,
            TokenType::Number,
        ]) {
            // Need to clone because self.previous returns immutable ref to the Token, thus we cannot move out the literal
            // Clone first then unwrap, since unwrap consumes the self value
            Ok(Expr::Literal(self.previous().literal.clone().unwrap()))
        } else if self.is_next_token(TokenType::LeftParen) {
            let expr = self.expression()?;

            // Check if there is a ")" to close the expression
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            // I dont think we should use self.current here
            Err(ParsingError::UnexpectedTokenError(
                (*self.current()).clone(),
                "Invalid token found while parsing expression",
            ))
        }
    }

    // Synchronize the tokens to approx the next valid token
    fn synchronize(&mut self) {
        // Loop till either EOF token or when one of the possible new start tokens is read
        // Where new start token, is a token that could indicate a new start where all previous syntax errors are behind it
        while !self.is_at_end() {
            // Advance to eat the current token AFTER making sure that we did not hit an EOF
            // Because if we called advance without checking for EOF with self.is_at_end() rust will panic when we unwrap Token after EOF
            //
            // Stop synchronize loop when semicolon is read. This assumes that in most cases, the error only cascades to a semicolon
            // This is a best case effort too, where it will fail when dealing with the semicolons in a for loop.
            // self.advance returns previous token, so it is chained here instead of making another call to self.previous()
            if self.advance().token_type == TokenType::Semicolon {
                return;
            }

            // Matching the other TokenTypes
            // When these token types are read, stop the synchronize loop
            match self.current().token_type {
                TokenType::Function
                | TokenType::Const
                | TokenType::If
                // | TokenType::For
                // | TokenType::Print
                | TokenType::While
                | TokenType::Return => return,
                _ => {}
            }
        }
    }
}
