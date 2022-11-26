use super::error::ParsingError;
use super::expr::Expr;
use super::parser_struct::Parser;
use super::stmt::Stmt;
use crate::literal::Literal;

use crate::token::Token;
use crate::token_type::TokenType;

/// Implementation of all the main methods used for parsing the vec of tokens into a vec of statements.
impl Parser {
    /// Consumes a token vector (takes ownership) to produce a statements vector (moved out)
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Vec<ParsingError>> {
        let mut parser = Parser {
            tokens,
            current_index: 0,
        };

        let mut statements: Vec<Stmt> = Vec::<Stmt>::new();
        let mut errors: Vec<ParsingError> = Vec::<ParsingError>::new();

        // On each loop, we scan a single token.
        while !parser.is_at_end() {
            // Get statement and based on output, push to either one of the vectors
            // Calling declaration parsing method, as declarations have the lowest precedence in the syntax grammar
            match parser.declaration() {
                Ok(stmt) => statements.push(stmt),

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
            // For variables, implementation with Null default value
            // let initial_value = if self.is_next_token(TokenType::Equal) {
            //     self.expression()?
            // } else {
            //     Expr::Literal(Literal::Null)
            // };
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
            // @todo try 'as_str' instead of the box then leak memory method
            // format!("Expect '(' after function name '{}'", name).as_str(),
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
            TokenType::Ignore => self.advance_and_call(Parser::ignore_statement),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after print expression")?;
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

        let line_number_of_closing_right_brace = self
            .consume(TokenType::RightBrace, "Expect '}' after block statement")?
            .line;

        // Empty block statements are not allowed, but will be checked in resolver instead.
        // The issue with checking here is that if this happen within a function definition, the error will bubble up to 'parse' method
        // and because of the synchronization function, it will continue resolving the function body, which means the closing '}' of the
        // function body is not consumed, and will create a false positive error later on when it is found, which may confuse user.
        // However if all is parsed, and error is handled by resolver, then this issue will not exist.
        // if statements.is_empty() {
        //     return Err(ParsingError::EmptyBlockStatement(
        //         line_number_of_closing_right_brace,
        //     ));
        // }

        Ok(Stmt::Block(
            statements,
            Some(line_number_of_closing_right_brace),
        ))
    }

    // @todo Support else if
    // while self.is_next_token(TokenType::ElseIf) {
    //     let else_branch = self.statement()?;
    // }
    // @todo Optimize by skipping blocks like "if (false)"
    fn if_statement(&mut self) -> Result<Stmt, ParsingError> {
        let line_number = self
            .consume(TokenType::LeftParen, "Expect `(` after 'if'")?
            .line;
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

        Ok(Stmt::If(
            condition,
            Box::new(true_branch),
            else_branch,
            line_number,
        ))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParsingError> {
        let line_number = self
            .consume(TokenType::LeftParen, "Expect `(` after 'while'")?
            .line;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect `)` after 'while' condition")?;

        let loop_body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(loop_body), line_number))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParsingError> {
        let line_number = self.previous().line;

        // Return value can either be an expression or Null if nothing is specified
        // @todo Alternatively, store value as Option<Value> in Stmt::Return and let interpreter deal with it
        let value = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Literal::Null)
        };

        self.consume(TokenType::Semicolon, "Expect `;` after return value.")?;
        Ok(Stmt::Return(Box::new(value), line_number))
    }

    fn ignore_statement(&mut self) -> Result<Stmt, ParsingError> {
        // If the line number is needed for debugging purposes, take here and store in Stmt::Ignore
        // let line_number = self.previous().line;
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after ignore statement.")?;
        Ok(Stmt::Ignore(expr))
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

    /// Not supporting assignment right now.
    /// This is a temporary assignment method that Errors out when an assignment is found.
    fn assignment(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.or()?;

        if self.is_next_token(TokenType::Equal) {
            // Assignments are not supported yet
            unimplemented!();

            // let token = self.previous().clone();

            // // Recursively calls itself as this is the top level expression parsing method. Can also call expression method but it will just call assignment method
            // let value = self.assignment()?;

            // match expr {
            //     Expr::Const(token, _) => Ok(Expr::Assign(token, Box::new(value), None)),
            //     Expr::Get(target, token) => Ok(Expr::Set(target, token, Box::new(value))),
            //     _ => Err(ParsingError::InvalidAssignmentError(token)),
            // }
        } else {
            Ok(expr)
        }
    }

    /// 'or' have a lower precedence than 'and'
    fn or(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.and()?;

        while self.is_next_token(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// 'and' have a higher precedence than 'or'
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

    /// Handles both function call and array access
    fn call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.primary()?;

        loop {
            // Only continue to treat this as a function call if there is an open Left Parenthesis which indicates a "call"
            if self.is_next_token(TokenType::LeftParen) {
                // @todo Might inline the function later
                expr = self.finish_call(expr)?;
            } else if self.is_next_token(TokenType::LeftBracket) {
                // Array access can be chained after function call expression, where the function call is expected to return an array
                expr = Expr::ArrayAccess(
                    // function call expression that is expected to return an array
                    Box::new(expr),
                    // Parse the index as an expression
                    Box::new(self.expression()?),
                );
                self.consume(
                    TokenType::RightBracket,
                    "Expect ']' after array access index expression",
                )?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Handle function calls by parsing for any arguments
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParsingError> {
        // Only create none empty vec for holding argument expressions if there are argument(s)
        let arguments: Vec<Expr> = if !self.check(TokenType::RightParen) {
            let mut _arguments: Vec<Expr> = Vec::new();

            // @todo Might want to limit argument size, based on spec, which can be a problem for VM implementations.
            // "Do while loop"
            _arguments.push(self.expression()?);
            while self.is_next_token(TokenType::Comma) {
                _arguments.push(self.expression()?);
            }

            _arguments
        } else {
            Vec::with_capacity(0)
        };

        // Check if there is a ")" to close the function call
        // The returned parenthesis token is used for line info later as needed during debug
        let parenthesis = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(Box::new(callee), arguments, parenthesis.clone()))
    }

    /// Primary expressions.
    /// Check for Identifier then Literal values True/False/Null then Strings/Numbers then Anonymous functions before moving on to grouped expressions.
    // @todo Boolean types can we still be represented using TokenType, so should literal Bool values be used?
    fn primary(&mut self) -> Result<Expr, ParsingError> {
        if self.is_next_token(TokenType::Identifier) {
            // Default "distance" is 0, as this value will be set by the resolver
            // @todo Move it out instead of cloning
            let identifier_expression = Expr::Const(self.previous().clone(), 0);

            // Check for LeftBracket to see if user is trying to access elements in an Array
            if self.is_next_token(TokenType::LeftBracket) {
                // Parse index as an expression
                let index_expression = self.expression()?;
                self.consume(
                    TokenType::RightBracket,
                    "Expect ']' after array access index expression",
                )?;

                Ok(Expr::ArrayAccess(
                    Box::new(identifier_expression), // identifier_expression is the array_identifier_expression
                    Box::new(index_expression),
                ))
            } else {
                Ok(identifier_expression)
            }
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
        } else if self.check(TokenType::LeftParen) {
            /*
                This block's condition is self.check instead of self.is_next_token,
                Because advance method should not be called, see parsing code docs for explaination

                This block parses for arrow functions and group expressions, where both starts with the LeftParen
                First this tries to parse for an arrow function by trying to parse for parameters,
                If that fails, then try to parse for group expression.

                There are 2 ways to parse for arrow function expressions,
                First is to try to parse for parameters, and if it failed it means that it is a group expression
                Second way is to loop until RightParen token, then check if the next token is an arrow token
                Using the first way here as it is probably more efficient.
            */

            // Calling parameters without any error message, because if the expression is not parameters,
            // then it will be treated as a group expression, rather then letting the error bubble up
            // So there is no need for any error messages, we are using parameters method almost as a
            // try/catch mechanism, where if tried and not parameters, then try to parse it as group expression.
            if let Ok(parameters) = self.parameters("") {
                self.consume(
                    TokenType::Arrow,
                    "Expect '=>' after parameters for anonymous arrow function",
                )?;

                // Arrow functions are single expression anonymous functions, where the single expression is the return value
                // So since the body is an expression, parse it as an expression before constructing a statement for AnonymousFunc stmt type
                // This 3 lines essentially desugar '() => expr' into 'function() { return expr; }'
                // Start parsing from "or" because the expression definitely cannot be an assignment
                let body = self.or()?;
                let return_statement = Stmt::Return(Box::new(body), self.previous().line);
                // @todo Create a small vec? Or something with size of just 1, since vec! does not..? See vec! implementation
                let block_statement = Stmt::Block(vec![return_statement], None);

                Ok(Expr::AnonymousFunc(Box::new(Stmt::AnonymousFunc(
                    parameters,
                    Box::new(block_statement),
                ))))
            } else {
                // TokenType::LeftParen is consumed when calling self.parameters()
                // So can just continuing parsing the inner expression
                // Start parsing from "or" because the expression definitely cannot be an assignment
                let expr = self.or()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
        } else if self.is_next_token(TokenType::LeftBracket) {
            // @todo Allow treating strings as an array, where "string"[2] === "r"
            // Parsing for array definition only, array access parsing is handled as an identifier expression

            // Get the vector elements in the array
            let elements = if self.check(TokenType::RightBracket) {
                // If array closed with no elements, return a Vec with 0 capacity to not allocate any memory
                Vec::with_capacity(0)
            } else {
                // Else create temporary vector to collect all elements before returning it
                let mut _elements: Vec<Expr> = Vec::new();

                // Do while loop
                _elements.push(self.expression()?);
                while self.is_next_token(TokenType::Comma) {
                    _elements.push(self.expression()?);
                }

                _elements
            };

            self.consume(TokenType::RightBracket, "Expect ']' to close the array")?;

            // @todo Use a better token then the closing ]
            Ok(Expr::Array(self.previous().clone(), elements))
        } else if self.is_at_end() {
            // Copied over from rlox
            // Not sure if this case will ever happen but just an extra safeguard for Unexpected Eof tokens
            Err(ParsingError::UnexpectedEofError(self.current().clone()))
        } else {
            // @todo Should the token be consumed here so that it would not have a issue doing error synchronization?
            Err(ParsingError::UnexpectedTokenError(
                self.current().clone(),
                "Invalid token found while parsing expression",
            ))
        }
    }

    // @todo Maybe use Vec<&Token> instead so dont have to clone every token once lifetime specifiers are added to Stmt
    /// Method to parse function parameters only. Works for all types of functions
    /// Caller to pass in error message to display if TokenType::LeftParen is not found at the beginning of expected parameter expression
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

    /// Synchronize the tokens to approx the next valid token
    fn synchronize(&mut self) {
        // Loop till either EOF token or when one of the possible new start tokens is read
        // Where new start token, is a token that could indicate a new start where all previous syntax errors are behind it
        while !self.is_at_end() {
            // Advance to eat the current token AFTER making sure that we did not hit an EOF
            // Because if we called advance without checking for EOF with self.is_at_end() rust will panic when we unwrap Token after EOF
            //
            // Stop synchronize loop when semicolon and these token types are read.
            // This assumes that in most cases, the error only cascades to a semicolon.
            // This is a best case effort too, where it will fail when dealing with the semicolons in a for loop.
            //
            // self.get_current_token_and_advance returns previous token, so it is chained here instead of making
            // another call to self.previous()
            match self.get_current_token_and_advance().token_type {
                TokenType::Semicolon
                | TokenType::Function
                | TokenType::Const
                | TokenType::If
                | TokenType::Print
                | TokenType::While
                | TokenType::Return => return,
                _ => {}
            }
        }
    }
}
