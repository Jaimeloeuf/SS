use super::parse_rule::{get_rule, Precedence, USIZE_TO_PRECEDENCE};
use super::CompileError;

use crate::chunk::Chunk;
use crate::compiler::Parser;
use crate::error::SSError;
use crate::opcode::OpCode;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;
use crate::value::Value;

#[derive(Debug)]
struct Local {
    name: String,
    depth: usize,
}

// The Compiler / Parser / Scanner structs are strung together,
// Compiler struct holds a Parser
// Parser struct holds a Scanner
// Scanner is created inside compile method, it is used to create Parser struct, which is used to create the Compiler struct
pub struct Compiler {
    pub chunk: Chunk,

    // Hold a parser so that it can be passed along to the methods easily instead of relying on global state like clox
    pub parser: Parser,

    // Vector of Locals to get at from the stack
    locals: Vec<Local>,

    // local_count field tracks how many locals are in scope / how many of those array slots are in use
    local_count: usize,

    // scope depth is the number of blocks surrounding the current bit of code we’re compiling.
    scope_depth: usize,
}

impl Compiler {
    // Returns Chunk that can be run immediately
    pub fn compile(source: String, chunk: Chunk) -> Result<Chunk, SSError> {
        // Create compiler struct internally instead of having a seperate method to create and compile.
        let mut compiler = Compiler {
            // Move chunk into the compiler struct, so that the methods can access it
            chunk,

            // Create default token structs using the derived default trait, since at the start current and previous tokens does not exists yet
            parser: Parser::new(
                // Create scanner using the source string
                Scanner::new(source),
                // Since parser current and previous fields hold tokens instead of Option<Token>, generate 2 default Tokens
                Token::default(),
                Token::default(),
            ),

            locals: Vec::<Local>::new(),
            local_count: 0,
            scope_depth: 0,
        };

        // Start by advancing the parser first, since Parser is created with default placeholder tokens
        // @todo Handle result return value
        compiler.parser.advance();

        // Keep parsing and compiling statements until EOF
        while !compiler.parser.match_next(TokenType::Eof) {
            // CompileError cannot be bubbled up as method's signature is Result<(), SSError>, so convert before returning error
            if let Err(e) = compiler.declaration() {
                return Err(SSError::CompileError(e));
            }
        }

        // @todo Fix the error message
        compiler
            .parser
            .consume(TokenType::Eof, "Expect end of expression".to_string());

        // Now that the chunk is filled with OpCodes after compilation, return it from Compiler struct to use with the VM
        Ok(compiler.chunk)
    }

    fn declaration(&mut self) -> Result<(), CompileError> {
        match &self.parser.current.token_type {
            TokenType::Const => self.advance_and_call(Compiler::const_declaration),
            TokenType::Function => self.advance_and_call(Compiler::function_declaration),

            // it is as a statement if it did not match any declaration tokens
            _ => self.statement(),
        }
    }

    // Method to parse a function, by parsing the function declaration which includes the name + parameter of a function,
    // Before handing it off to self.block_statement method to compile the function body as a block statement
    fn function_declaration(&mut self) -> Result<(), CompileError> {
        // Consume identifier token before parsing for the function's identifier/name string
        self.parser
            .consume(TokenType::Identifier, "Expect function name".to_string());

        // Get name/identifier string of function
        let function_name = self.parse_identifier_string();

        // Only works for local scope
        self.declare_const(&function_name)?;

        self.parser.consume(
            TokenType::LeftParen,
            "Expect '(' after function identifier".to_string(),
        );

        // @todo WIP Parameters
        // Get the vector of parameters of the function
        let parameters = if self.parser.check(TokenType::RightParen) {
            // If function definition closed with no parameters, return a Vec with 0 capacity to not allocate any memory
            Vec::with_capacity(0)
        } else {
            // Else create temporary vector to collect all parameters before returning it
            let mut _parameters: Vec<Token> = Vec::new();

            // self.expression()
            self.parser.consume(
                TokenType::Identifier,
                "Expected parameter identifier".to_string(),
            );

            // Do while loop
            _parameters.push(self.parser.previous.clone());
            while self.parser.match_next(TokenType::Comma) {
                self.parser.consume(
                    TokenType::Identifier,
                    "Expected parameter identifier".to_string(),
                );
                _parameters.push(self.parser.previous.clone());
            }

            _parameters
        };

        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after function parameters".to_string(),
        );

        // Emit function as a constant value
        self.emit_constant(Value::Fn(self.chunk.codes.len() + 3));

        // Only works for global scope
        self.define_const(function_name);

        // Add JUMP to jump over codes of the function body, as the function is being defined and not being executed/called yet
        let jump_over_fn_body: usize = self.emit_jump(OpCode::JUMP(0));

        // Need to consume the LeftBrace before calling block_statement method as it assumes that it is already consumed
        self.parser.consume(
            TokenType::LeftBrace,
            "Expect '{' before function body".to_string(),
        );

        // Function body is compiled just like any other block statement
        self.block_statement()?;

        // @todo WIP return and return values...
        // Add a default return to mark the end of the function body
        // Usually there will be a return, but in case the function does not have any, this will break out of the function
        // Default return is a Null, since a function call is an expression and always expects a value to be left on stack
        self.emit_constant(Value::Null);
        self.emit_code(OpCode::RETURN);

        // Patch the jump over function body once it has been compiled
        self.patch_jump(jump_over_fn_body)?;

        Ok(())
    }

    fn const_declaration(&mut self) -> Result<(), CompileError> {
        // Consume the identifier token before parsing for the const's identifier string
        self.parser
            .consume(TokenType::Identifier, "Expect const name".to_string());

        let const_name = self.parse_identifier_string();

        // Only works for local scope
        self.declare_const(&const_name)?;

        // @todo Should not have this right, all const must be initialized
        if self.parser.match_next(TokenType::Equal) {
            self.expression()?;
        } else {
            self.emit_constant(Value::Null);
        }

        self.parser.consume(
            TokenType::Semicolon,
            "Expect ';' after const declaration".to_string(),
        );

        // Only works for global scope
        self.define_const(const_name);

        Ok(())
    }

    // Return previous token in parser as a String used as an identifier
    fn parse_identifier_string(&mut self) -> String {
        self.parser.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length]
            .parse::<String>()
            .unwrap()
    }

    fn define_const(&mut self, const_name: String) {
        // @todo Skip if none global scope
        if self.scope_depth > 0 {
            return;
        }

        self.emit_code(OpCode::IDENTIFIER(const_name));
    }

    fn declare_const(&mut self, identifier: &String) -> Result<(), CompileError> {
        // @todo Skip if global scope
        if self.scope_depth != 0 {
            // Run identifier check to make sure it is unused in current scope, if there are local identifiers already
            // Check from the last element in locals to the first, only stopping when scope ends or no more locals
            for local in (&self.locals).into_iter().rev() {
                // Check to ensure still in the same scope
                if local.depth < self.scope_depth {
                    break;
                }

                // Ensure that the identifier name is unique in current scope
                if identifier == &local.name {
                    // @todo Include line info
                    return Err(CompileError::IdentifierAlreadyUsed(identifier.clone()));
                }
            }

            self.add_local(identifier.clone());
        }

        // Return Ok variant with unit type
        Ok(())
    }

    fn add_local(&mut self, identifier: String) {
        self.local_count += 1;
        self.locals.push(Local {
            name: identifier,
            depth: self.scope_depth,
        });
    }

    fn statement(&mut self) -> Result<(), CompileError> {
        match &self.parser.current.token_type {
            TokenType::Print => self.advance_and_call(Compiler::print_statement),
            TokenType::LeftBrace => self.advance_and_call(Compiler::block_statement),
            TokenType::Return => self.advance_and_call(Compiler::return_statement),
            TokenType::If => self.advance_and_call(Compiler::if_statement),
            TokenType::While => self.advance_and_call(Compiler::while_statement),

            // it is as an expression statement if it did not match any statement tokens
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<(), CompileError> {
        self.expression()?;

        self.parser.consume(
            TokenType::Semicolon,
            "Expect ';' after print statement".to_string(),
        );

        self.emit_code(OpCode::PRINT);

        Ok(())
    }

    fn block_statement(&mut self) -> Result<(), CompileError> {
        // Create a new scope by incrementing compiler's scope depth
        self.scope_depth += 1;

        // Keep parsing/compiling as long as it is not the closing right brace or an unexpected EOF yet
        while !self.parser.check(TokenType::RightBrace) && !self.parser.check(TokenType::Eof) {
            self.declaration()?;
        }

        self.parser
            .consume(TokenType::RightBrace, "Expect '}' after block".to_string());

        // Destroy the current block scope by decrementing compiler's scope depth
        self.scope_depth -= 1;

        // Delete local identifier's values whose lifetime ends in current scope from locals vector, and emit opcode to delete from stack
        // Can unwrap last() value directly because len has already been checked to be bigger than 0
        //
        // Instead of popping values of stack 1 by 1 using multiple pop opcodes,
        // Use POP_N(usize) opcode, to pop N number of values of the stack with a single opcode to make runtime faster
        let mut number_of_pops = 0;
        while self.locals.len() > 0 && self.locals.last().unwrap().depth > self.scope_depth {
            // Remove the local from compiler's locals vector too
            self.locals.pop();
            number_of_pops += 1;
        }

        // Only emit POP_N instruction if there are locals to pop off the stack
        if number_of_pops > 0 {
            self.emit_code(OpCode::POP_N(number_of_pops));
        }

        Ok(())
    }

    // @todo Error when outside of a function body. Should be a compile error instead of runtime error
    // Return statements can happen anywhere in a function body to stop execution
    fn return_statement(&mut self) -> Result<(), CompileError> {
        // If semicolon read a.k.a no return expression, compile "return;" as shorthand for "return null;"
        if self.parser.check(TokenType::Semicolon) {
            self.emit_constant(Value::Null);
        } else {
            self.expression()?;
        }

        self.parser.consume(
            TokenType::Semicolon,
            "Expect ';' after return statement".to_string(),
        );

        self.emit_code(OpCode::RETURN);

        Ok(())
    }

    fn resolve_local(&mut self, identifier: &str) -> Result<usize, CompileError> {
        // Reverse to allow identifier shadowing in child scope
        // loop_index starts from 0..(self.locals.len() - 1) where 0 refers to the last element in the vec
        for (loop_index, local) in (&self.locals).into_iter().rev().enumerate() {
            if identifier == local.name {
                // Calculate stack index, using length of vector - 1 - loop_index
                // -1 from length as vec index starts from 0, and -loop_index to get actual stack index since loop is reversed
                return Ok(self.locals.len() - 1 - loop_index);
            }
        }

        // This assumes error, but in Clox it means try looking for a global variable instead
        eprintln!("Identifier not available in local scope");
        return Err(CompileError::IdentifierAlreadyUsed(identifier.to_string()));
    }

    fn if_statement(&mut self) -> Result<(), CompileError> {
        self.parser.consume(
            TokenType::LeftParen,
            "Expect '(' after 'if' keyword".to_string(),
        );
        // Parse the condition expression
        self.expression()?;
        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after 'if' condition".to_string(),
        );

        let then_jump: usize = self.emit_jump(OpCode::JUMP_IF_FALSE(0));
        // POP opcode to discard condition value from stack
        self.emit_code(OpCode::POP);
        self.statement()?;

        let else_jump: usize = self.emit_jump(OpCode::JUMP(0));

        self.patch_jump(then_jump)?;

        // POP opcode to discard condition value from stack
        self.emit_code(OpCode::POP);

        if self.parser.match_next(TokenType::Else) {
            self.statement()?;
        }
        self.patch_jump(else_jump)?;

        Ok(())
    }

    fn while_statement(&mut self) -> Result<(), CompileError> {
        // Store the chunk’s current opcode count to record the opcodes offset right before compiling the condition expression
        let loop_start: usize = self.chunk.codes.len();

        self.parser.consume(
            TokenType::LeftParen,
            "Expect '(' after 'while' keyword".to_string(),
        );
        // Parse the condition expression
        self.expression()?;
        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after 'while' condition".to_string(),
        );

        let exit_jump: usize = self.emit_jump(OpCode::JUMP_IF_FALSE(0));
        // POP opcode to discard condition value from stack
        self.emit_code(OpCode::POP);
        self.statement()?;

        // Calculate the opcode count difference between current length after compiling loop body and start of loop
        let offset = self.chunk.codes.len() - loop_start;

        // Although this can be implemented with JUMP(-offset), alot more work needs to be done in the VM to support negative offsets
        // This is because most offset calculation and things like the VM's Instruction Pointer are all usize
        self.emit_code(OpCode::LOOP(offset));

        self.patch_jump(exit_jump)?;

        // POP opcode to discard condition value from stack
        self.emit_code(OpCode::POP);

        Ok(())
    }

    // An expression statement is an expression followed by a semicolon.
    // They’re how you write an expression in a context where a statement is expected.
    // Usually, it’s so that you can call a function or evaluate an assignment for its side effect
    // An expression statement evaluates the expression and discards the result from the stack
    fn expression_statement(&mut self) -> Result<(), CompileError> {
        self.expression()?;

        self.parser.consume(
            TokenType::Semicolon,
            "Expect ';' after expression".to_string(),
        );

        // POP opcode to discard result from the stack
        // This assumes every single type of expression will always leave exactly one value on the stack once executed
        // And since this is a single expression statement, the value on stack is not needed, thus discarded
        self.emit_code(OpCode::POP);

        Ok(())
    }

    /*
        ============= Expression compiler methods =============

        Methods to parse and compile expressions are public,
        as they are referenced in the RULES_TABLE which will be used by parse_precedence
        method to call expression compiler methods recursively as needed.
    */

    fn expression(&mut self) -> Result<(), CompileError> {
        self.parse_precedence(Precedence::Assignment)
    }

    /// Compile an identifier use/lookup into either a local value ora global scope identifier lookup opcode
    pub fn identifier_lookup(&mut self) -> Result<(), CompileError> {
        let identifier = self.parse_identifier_string();

        // Handling identifiers in local scopes differently from global scope identifiers
        // @todo Merge these
        match self.resolve_local(&identifier) {
            Ok(stack_index) => self.emit_code(OpCode::GET_LOCAL(stack_index)),
            Err(_) => self.emit_code(OpCode::IDENTIFIER_LOOKUP(identifier)),
        };

        Ok(())
    }

    // Method to compile function calls
    // @todo Handle calls with arguments
    pub fn call(&mut self) -> Result<(), CompileError> {
        self.parser.consume(
            TokenType::RightParen,
            "Expected ')' after function arguments".to_string(),
        );
        self.emit_code(OpCode::CALL);
        Ok(())
    }

    // @todo Add error checks when unwrapping
    pub fn number(&mut self) -> Result<(), CompileError> {
        let value: f64 = self.parser.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length]
            .parse::<f64>()
            .unwrap();
        self.emit_constant(Value::Number(value));

        Ok(())
    }

    // @todo Add error checks when unwrapping
    pub fn string(&mut self) -> Result<(), CompileError> {
        let value: String = self.parser.scanner.source[
                // Plus 1 from starting char to skip the " double quote literal
                // Minus 1 to skip the " double quote literal after the string literal
                self.parser.previous.start + 1 ..
                self.parser.previous.start + self.parser.previous.length - 1
            ]
            .parse::<String>()
            .unwrap();
        self.emit_constant(Value::String(value));

        Ok(())
    }

    pub fn grouping(&mut self) -> Result<(), CompileError> {
        self.expression()?;
        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after expression".to_string(),
        );

        Ok(())
    }

    pub fn unary(&mut self) -> Result<(), CompileError> {
        // Remember the operator because the next call to parse_precedence moves the parser forward
        // Need to clone here instead of taking a immutable ref because self.parse_precedence needs a mutable ref to self
        let operator_type: TokenType = self.parser.previous.token_type.clone();

        // Compile the operand
        self.parse_precedence(Precedence::Unary)?;

        // Emit the operator instruction.
        Ok(match operator_type {
            TokenType::Bang => self.emit_code(OpCode::NOT),
            TokenType::Minus => self.emit_code(OpCode::NEGATE),

            // Unreachable
            _ => return Err(CompileError::InvalidOperatorType(operator_type)),
        })
    }

    pub fn binary(&mut self) -> Result<(), CompileError> {
        // Remember the operator because the next call to parse_precedence moves the parser forward
        // Need to clone here instead of taking a immutable ref because self.parse_precedence needs a mutable ref to self
        let operator_type: TokenType = self.parser.previous.token_type.clone();

        // Parse/Compile right operand first, so that opcode will execute before operator code,
        // which for binary arithmetic operators needs the values on the stack already.
        // Get next precedence enum variant and parse it
        self.parse_precedence(
            USIZE_TO_PRECEDENCE[get_rule(&operator_type).precedence as usize + 1],
        )?;

        // Alternative is to use method that relies on unsafe mem::transmute code
        // self.parse_precedence(Precedence::from_usize(
        //     get_rule(&operator_type).precedence as usize + 1,
        // ));

        // Emit the operator's OpCode
        Ok(match operator_type {
            TokenType::Plus => self.emit_code(OpCode::ADD),
            TokenType::Minus => self.emit_code(OpCode::SUBTRACT),
            TokenType::Star => self.emit_code(OpCode::MULTIPLY),
            TokenType::Slash => self.emit_code(OpCode::DIVIDE),

            TokenType::BangEqual => self.emit_code(OpCode::NOT_EQUAL),
            TokenType::EqualEqual => self.emit_code(OpCode::EQUAL),
            TokenType::Greater => self.emit_code(OpCode::GREATER),
            TokenType::GreaterEqual => self.emit_code(OpCode::GREATER_EQUAL),
            TokenType::Less => self.emit_code(OpCode::LESS),
            TokenType::LessEqual => self.emit_code(OpCode::LESS_EQUAL),

            // Unreachable
            _ => return Err(CompileError::InvalidOperatorType(operator_type)),
        })
    }

    /*
        The 2 operands of 'and' will be typed check to be bool
        The first operand on the left hand side, is checked by JUMP_IF_FALSE opcode, which requires bool conditionals
        The second operand on the right hand side, is checked by TYPE_CHECK_BOOL opcode, which throws runtime error if last value on stack is not bool
    */
    pub fn and(&mut self) -> Result<(), CompileError> {
        let end_jump: usize = self.emit_jump(OpCode::JUMP_IF_FALSE(0));

        // POP opcode to discard condition value from stack, which will be the left hand side expression of the 'and' keyword
        self.emit_code(OpCode::POP);

        // Parse/Compile the right hand side expression
        self.parse_precedence(Precedence::And)?;

        // Patch jump to jump/skip the right hand side operand expression opcodes if the left hand side operand is Bool(false)
        self.patch_jump(end_jump)?;

        // Add TYPE_CHECK_BOOL opcode to type check if the right hand side operand expression evaluates to a Bool(_)
        self.emit_code(OpCode::TYPE_CHECK_BOOL);

        Ok(())
    }

    /*
        The 2 operands of 'or' will be typed check to be bool
        The first operand on the left hand side, is checked by JUMP_IF_FALSE opcode, which requires bool conditionals
        The second operand on the right hand side, is checked by TYPE_CHECK_BOOL opcode, which throws runtime error if last value on stack is not bool

        @todo Because of how we compile this, if the first value is true, the second value will not be type checked
    */
    pub fn or(&mut self) -> Result<(), CompileError> {
        let else_jump: usize = self.emit_jump(OpCode::JUMP_IF_FALSE(0));
        let end_jump: usize = self.emit_jump(OpCode::JUMP(0));

        // If left hand side expression is Bool(false), jump to the POP instruction emitted below to pop it from stack.
        self.patch_jump(else_jump)?;

        // POP opcode to discard condition value from stack, which will be the left hand side expression of the 'or' keyword
        // This only executes if left hand side expression evaluates to Bool(false), to pop it off before evaluating the right hand side expression
        self.emit_code(OpCode::POP);

        // Parse/Compile the right hand side expression
        self.parse_precedence(Precedence::Or)?;

        // Patch jump to jump/skip the right hand side operand expression opcodes if the left hand side operand is Bool(true)
        self.patch_jump(end_jump)?;

        // Add TYPE_CHECK_BOOL opcode to type check if the right hand side operand expression evaluates to a Bool(_)
        self.emit_code(OpCode::TYPE_CHECK_BOOL);

        Ok(())
    }

    pub fn literal(&mut self) -> Result<(), CompileError> {
        Ok(match &self.parser.previous.token_type {
            // Optimize by using special opcodes, like OpCode::True to load True onto stack directly instead of reading from CONSTANT(val)
            TokenType::True => self.emit_constant(Value::Bool(true)),
            TokenType::False => self.emit_constant(Value::Bool(false)),
            TokenType::Null => self.emit_constant(Value::Null),

            _ => {
                return Err(CompileError::InvalidOperatorType(
                    self.parser.previous.token_type.clone(),
                ))
            }
        })
    }

    /* ============= End of Expression compiler methods ============= */

    // Parse expression by using the TokenType to get a ParseRule's parse/compile method
    // Continues to parse/compile infix operators if the precedence level is low enough
    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompileError> {
        // Shadow precedence variable to convert it from enum variant to usize for numerical comparison later
        let precedence = precedence as usize;

        // @todo Handle result variant
        // Read the next token
        self.parser.advance();

        // Look up corresponding ParseRule of the previous token's TokenType, and match to use the prefix parser
        match get_rule(&self.parser.previous.token_type).prefix {
            // Alternative syntax for self.prefix_rule() where prefix_rule is a variable function pointer
            // Some(prefix_rule) => prefix_rule(self),
            Some(prefix_rule) => prefix_rule(self, false)?,

            // If there is no prefix parser, then the token must be a syntax error
            None => {
                return Err(CompileError::MissingParser(
                    "Expect expression. No prefix parser".to_string(),
                ))
            }
        };

        // After parsing the prefix expression, which may consume more tokens this look for an infix parser for the next token.
        // If there is one, it means the prefix expression this just compiled might be an operand for it,
        // BUT ONLY if the call to parse_precedence() has a precedence that is low enough to permit that infix operator.
        // To test if it is low enough, convert ParseRule's precedence into its usize discriminant to compare with the precedence passed in
        while precedence <= get_rule(&self.parser.current.token_type).precedence as usize {
            // @todo Handle result variant
            // Read the next token
            self.parser.advance();

            // Look up corresponding ParseRule of the previous token's TokenType, and match to use the infix parser
            match get_rule(&self.parser.previous.token_type).infix {
                // Alternative syntax for self.infix_rule() where infix_rule is a variable function pointer
                // Some(infix_rule) => infix_rule(self),
                Some(infix_rule) => infix_rule(self, false)?,

                // If there is no prefix parser, then the token must be a syntax error
                None => {
                    return Err(CompileError::MissingParser(
                        "Expect expression. No infix parser".to_string(),
                    ))
                }
            }
        }

        Ok(())
    }
}
