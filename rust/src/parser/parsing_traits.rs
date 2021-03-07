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
            current_index: 0,
        };

        println!("Processing '{}' tokens", parser.tokens.len());

        let mut statements: Vec<Stmt> = Vec::<Stmt>::new();
        let mut errors: Vec<ParsingError> = Vec::<ParsingError>::new();

        // On each loop, we scan a single token.
        while !parser.is_at_end() {
            // Get statement and based on output, push to either one of the vectors
            // Calling declaration parsing method, as declarations have the lowest precedence in the syntax grammar
            match parser.declaration() {
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

    /* ==========================  Start of declaration methods  ========================== */

    fn declaration(&mut self) -> Result<Stmt, ParsingError> {
        // Call the different declaration parsing methods base on token_type
        // Else if not the initial token of a declaration, it must be either a normal statement or an expression
        // Pass control to statement method to continue parsing for statment or expression
        // Using advance_and_call to call advance method before calling method to eat the matched token
        match &self.current().token_type {
            TokenType::Const => self.advance_and_call(Parser::const_declaration),
            TokenType::Function => self.advance_and_call(Parser::function_declaration),
            _ => self.statement(),
        }
    }

    fn const_declaration(&mut self) -> Result<Stmt, ParsingError> {
        let name = self.consume(TokenType::Identifier, "Expected name for constant")?;
        // @todo Will fail without this clone???? cannot even clone later????
        let name = name.clone();

        if self.is_next_token(TokenType::Equal) {
            let initial_value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after const declaration")?;
            Ok(Stmt::Const(name, initial_value))
        } else {
            // @todo Should we allow unassigned? But const.... cannot reassign already... so this should only be done for variables if any
            // Err if missing Equal token
            Err(ParsingError::UnexpectedTokenError(
                self.current().clone(),
                "Expected 'Equal' token for assignment after const keyword's identifier",
            ))
        }
    }

    fn function_declaration(&mut self) -> Result<Stmt, ParsingError> {
        let name = self.consume(TokenType::Identifier, "Expected name for function")?;
        let name = name.clone();

        // Get the function parameters
        let parameters: Vec<Token> = self.parameters(
            // @todo Makes String into &'static str by LEAKING THE MEMORY!!! --> https://stackoverflow.com/a/30527289/13137262
            Box::leak(format!("Expect '(' after function name '{}'", name).into_boxed_str()),
        )?;

        // Wording might just not be function body if we support methods too
        self.consume(TokenType::LeftBrace, "Expected '{' before function body.")?;

        let body = self.block_statement()?;
        Ok(Stmt::Func(name.clone(), parameters, Box::new(body)))
    }

    /* ==========================  End of declaration methods  ========================== */

    /* ==========================  Start of statement methods  ========================== */

    fn statement(&mut self) -> Result<Stmt, ParsingError> {
        // Call the different statement parsing methods base on token_type
        // Else if not a statement token, it must be a expression statement
        // Pass control to expression_statement method to continue parsing for an expression
        // Using advance_and_call to call advance method before calling method to eat the matched token
        match &self.current().token_type {
            TokenType::Print => self.advance_and_call(Parser::print_statement),
            TokenType::LeftBrace => self.advance_and_call(Parser::block_statement),
            TokenType::If => self.advance_and_call(Parser::if_statement),
            TokenType::While => self.advance_and_call(Parser::while_statement),
            // TokenType::For => self.advance_and_call(Parser::for_statement),
            TokenType::Return => self.advance_and_call(Parser::return_statement),
            _ => self.expression_statement(),
        }
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

    fn block_statement(&mut self) -> Result<Stmt, ParsingError> {
        let mut statements: Vec<Stmt> = Vec::<Stmt>::new();

        // Parse statements 1 by 1 till either end of block statement or Eof
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block statement")?;
        Ok(Stmt::Block(statements))
    }

    // @todo Support else if
    // while self.is_next_token(TokenType::ElseIf) {
    //     let else_branch = self.statement()?;
    // }
    // @todo Optimize by skipping blocks like "if (false)"
    fn if_statement(&mut self) -> Result<Stmt, ParsingError> {
        self.consume(TokenType::LeftParen, "Expect `(` after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect `)` after 'if' condition")?;

        let true_branch = self.statement()?;

        // Only parse for an else branch if there is a Else token
        // Wrap in an Option variant as thats what the Stmt variant expects
        let else_branch = if self.is_next_token(TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, Box::new(true_branch), else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParsingError> {
        self.consume(TokenType::LeftParen, "Expect `(` after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect `)` after 'while' condition")?;

        let loop_body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(loop_body)))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParsingError> {
        // @todo Why need to clone previous? Does Stmt::Return really need the token?
        let keyword = self.previous().clone();

        // Return value can either be an expression or Null if nothing is specified
        let value = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Literal::Null)
        };

        self.consume(TokenType::Semicolon, "Expect `;` after return value.")?;
        Ok(Stmt::Return(keyword, Box::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }

    /* ==========================  End of statement methods  ========================== */

    fn expression(&mut self) -> Result<Expr, ParsingError> {
        self.assignment()
    }

    // Not supporting assignment first
    // fn assignment(&mut self) -> Result<Expr, ParsingError> {
    //     let expr = self.or()?;

    //     if self.is_next_token(TokenType::Equal) {
    //         let token = self.previous().clone();
    //         // Recursively calls itself as this is the top level expression parsing method. Can also call expression method but it will just call assignment method
    //         let value = self.assignment()?;

    //         match expr {
    //             Expr::Const(token, _) => Ok(Expr::Assign(token, Box::new(value), None)),
    //             Expr::Get(target, token) => Ok(Expr::Set(target, token, Box::new(value))),
    //             _ => Err(ParsingError::InvalidAssignmentError(token)),
    //         }
    //     } else {
    //         Ok(expr)
    //     }
    // }
    // Temporary assignment method that Errors out when assignment is found
    fn assignment(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.or()?;

        if self.is_next_token(TokenType::Equal) {
            Err(ParsingError::InternalError(
                self.current().line,
                "Assignments are not supported yet",
            ))
        } else {
            Ok(expr)
        }
    }

    // 'or' have a lower precedence than 'and'
    fn or(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.and()?;

        while self.is_next_token(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.equality()?;

        while self.is_next_token(TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.primary()?;

        loop {
            // Only continue to treat this as a function call if there is an open Left Parenthesis which indicates a "call"
            if self.is_next_token(TokenType::LeftParen) {
                // @todo Might inline the function later
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    // Refactored out function to handle function calls by parsing their arguements
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParsingError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(TokenType::RightParen) {
            // @todo Might want to limit arguement size, based on spec, which can be a problem for VM implementations.
            // "Do while loop"
            arguments.push(self.expression()?);
            while self.is_next_token(TokenType::Comma) {
                arguments.push(self.expression()?);
            }
        }

        // @todo Is there a need for the returned parenthesis?
        // Check if there is a ")" to close the function call
        let parenthesis = self.consume(TokenType::RightParen, "Expect ')' after arguements.")?;

        Ok(Expr::Call(Box::new(callee), arguments, parenthesis.clone()))
    }

    // Primary expressions
    // Check for Identifier then Literal values True/False/Null then Strings/Numbers then Anonymous functions before moving on to grouped expressions
    // @todo Boolean types can we still be represented using TokenType, so should literal Bool values be used?
    fn primary(&mut self) -> Result<Expr, ParsingError> {
        if self.is_next_token(TokenType::Identifier) {
            // Default "distance" is 0, as this value will be set by the resolver
            // @todo Move it out instead of cloning
            Ok(Expr::Const(self.previous().clone(), 0))
        } else if self.is_next_token(TokenType::True) {
            Ok(Expr::Literal(Literal::Bool(true)))
        } else if self.is_next_token(TokenType::False) {
            Ok(Expr::Literal(Literal::Bool(false)))
        } else if self.is_next_token(TokenType::Null) {
            Ok(Expr::Literal(Literal::Null))
        } else if self.is_next_token_any_of_these(vec![TokenType::Str, TokenType::Number]) {
            // Need to clone because self.previous returns immutable ref to the Token, thus we cannot move out the literal
            // Clone first then unwrap, since unwrap consumes the self value
            // @todo Move the literal value out instead of cloning it
            Ok(Expr::Literal(self.previous().literal.clone().unwrap()))
        } else if self.is_next_token(TokenType::Function) {
            // Parse for Anonymous block function type 'function() { ... }'

            // Get the function parameters
            let parameters: Vec<Token> =
                self.parameters("Expect '(' after function keyword for anonymous functions")?;

            self.consume(
                TokenType::LeftBrace,
                "Expect '{' before anonymous block function body.",
            )?;

            // Just like named functions, parse function body as a block statement
            let body = self.block_statement()?;
            Ok(Expr::AnonymousFunc(Box::new(Stmt::AnonymousFunc(
                parameters,
                Box::new(body),
            ))))
        } else if self.is_next_token(TokenType::LeftParen) {
            let expr = self.expression()?;

            // Check if there is a ")" to close the expression
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else if self.is_at_end() {
            // Copied over from rlox
            // Not sure if this case will ever happen but just an extra safeguard for Unexpected Eof tokens
            Err(ParsingError::UnexpectedEofError(self.current().clone()))
        } else {
            // I dont think we should use self.current here
            Err(ParsingError::UnexpectedTokenError(
                self.current().clone(),
                "Invalid token found while parsing expression",
            ))
        }
    }

    // @todo Maybe use Vec<&Token> instead so dont have to clone every token once lifetime specifiers are added to Stmt
    // Method to parse function parameters only. Works for all types of functions
    // Caller to pass in error message to display if TokenType::LeftParen is not found at the beginning of expected parameter expression
    fn parameters(
        &mut self,
        missing_left_paren_error: &'static str,
    ) -> Result<Vec<Token>, ParsingError> {
        // Use caller provided string as different types of functions will pass in different error messages
        self.consume(TokenType::LeftParen, missing_left_paren_error)?;

        // Get the vector of parameters of the function
        let parameters = if self.check(TokenType::RightParen) {
            // If function parameter closed with no parameters, return a Vec with 0 capacity to not allocate any memory
            Vec::with_capacity(0)
        } else {
            // Else create temporary vector to collect all parameters before returning it
            let mut _parameters: Vec<Token> = Vec::new();

            // Do while loop
            _parameters.push(
                self.consume(TokenType::Identifier, "Expected parameter name")?
                    .clone(),
            );
            while self.is_next_token(TokenType::Comma) {
                _parameters.push(
                    self.consume(TokenType::Identifier, "Expected parameter name")?
                        .clone(),
                );
            }

            _parameters
        };

        self.consume(
            TokenType::RightParen,
            "Expect ')' after function parameters.",
        )?;

        Ok(parameters)
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
