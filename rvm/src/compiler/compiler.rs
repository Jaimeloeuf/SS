use std::collections::hash_map::HashMap;

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

// @todo Refactor this out into its own module
#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub depth: usize,
}

// @todo Refactor this out into its own module
// The Compiler / Parser / Scanner structs are strung together,
// Compiler struct holds a Parser
// Parser struct holds a Scanner
// Scanner is created inside compile method, it is used to create Parser struct, which is used to create the Compiler struct
pub struct Compiler {
    /// Generated opcodes are emitted into this chunk
    pub chunk: Chunk,

    /// Hold a parser so that it can be passed along to the methods easily instead of relying on global state like clox
    pub parser: Parser,

    /// Vector of Locals to get at from the stack
    pub locals: Vec<Local>,

    /// HashMap<function_string_identifier, parameter_count>
    ///
    /// Used to ensure number of arguments match number of parameters defined
    pub functions_parameter_count: HashMap<String, usize>,

    /// scope depth is the number of blocks surrounding the current bit of code being compiling.
    pub scope_depth: usize,

    /// function scopes is the number of function bodies surrounding the current bit of code being compiling.
    /// Used to track if compiler is currently compiling a function body
    pub function_scopes: usize,
}

impl Compiler {
    /// Returns a 'Chunk' that can be run immediately
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
            functions_parameter_count: HashMap::<String, usize>::new(),
            scope_depth: 0,
            function_scopes: 0,
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

    /* ================== Declaration compiler methods ================== */

    fn declaration(&mut self) -> Result<(), CompileError> {
        match &self.parser.current.token_type {
            TokenType::Const => self.advance_and_call(Compiler::const_declaration),
            TokenType::Function => self.advance_and_call(Compiler::function_declaration),

            // it is as a statement if it did not match any declaration tokens
            _ => self.statement(),
        }
    }

    /// Method to parse a function, by parsing the function declaration which includes the name + parameter of a function,
    /// Before handing it off to self.block_statement method to compile the function body as a block statement
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

        let parameter_identifiers: Vec<String> = if self.parser.check(TokenType::RightParen) {
            // If function definition closed with no parameters, return a Vec with 0 capacity to not allocate any memory
            Vec::with_capacity(0)
        } else {
            // Else create temporary vector to collect all parameters before returning it
            let mut _parameters: Vec<String> = Vec::new();

            /* 'Do while' loop to consume the function parameters as identifiers before parsing and storing its string */
            self.parser.consume(
                TokenType::Identifier,
                "Expected parameter identifier".to_string(),
            );
            _parameters.push(self.parse_identifier_string());

            while self.parser.match_next(TokenType::Comma) {
                self.parser.consume(
                    TokenType::Identifier,
                    "Expected parameter identifier".to_string(),
                );
                _parameters.push(self.parse_identifier_string());
            }

            _parameters
        };

        // Store the function name and its parameter count into hashmap, to check if arg count is matching
        self.functions_parameter_count
            .insert(function_name.clone(), parameter_identifiers.len());

        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after function parameters".to_string(),
        );

        // Emit function as a constant value
        // Here the opcode index of the chunk pointing to this function body will be calculated
        // The opcode index can be calculated using the current opcode index, and a few skips for the comming instructions like JUMP
        //
        // A -1 is NOT NEEDED even though the base value uses the length of the vector rather than the index,
        // because this particular constant loading code, has not been generated yet.
        //
        // +1 Skip this constant opcode that loads the function onto stack
        // +1 OR +0 Only add a skip if function is defined in global scope, which means an additional opcode to set value to identifier
        // +1 to skip the jump_over_fn_body, since now we actually want to execute the fn body code
        //
        // So the final value is 'current_opcode_index + 3' if function defined in global scope, else it is 'current_opcode_index + 2'
        self.emit_constant(Value::Fn(
            self.chunk.codes.len() + if self.scope_depth == 0 { 3 } else { 2 },
        ));

        // Only works for global scope
        self.define_const(function_name);

        // Add JUMP to jump over codes of the function body, as the function is being defined and not being executed/called yet
        let jump_over_fn_body: usize = self.emit_jump(OpCode::JUMP(0));

        // Need to consume the LeftBrace before calling block_statement method as it assumes that it is already consumed
        self.parser.consume(
            TokenType::LeftBrace,
            "Expect '{' before function body".to_string(),
        );

        // Increment function scopes before compiling the function body
        self.function_scopes += 1;

        // Function body is compiled just like any other block statement
        self.block_statement()?;

        // Decrement number of function scopes once function body is compiled
        self.function_scopes -= 1;

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

    /// Generate a identifier/value pair if code is in global scope
    fn define_const(&mut self, const_name: String) {
        // @todo Skip if none global scope
        if self.scope_depth > 0 {
            return;
        }

        self.emit_code(OpCode::IDENTIFIER(const_name));
    }

    /// Generate a identifier/value pair if code is in local scope
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

    /* ============= End of Declaration compiler methods ============= */

    /* ================== Statement compiler methods ================== */

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

    /// Block statements to create new scopes, used by itself or with a conditional or loop
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

        self.pop_out_of_scope_locals_from_stack();

        Ok(())
    }

    // @todo Error when outside of a function body. Should be a compile error instead of runtime error
    /// Compile user's return statements, that can happen anywhere in a function body to stop execution. NOT USED for default return in function
    fn return_statement(&mut self) -> Result<(), CompileError> {
        // Error if return is found but compiler is not enclosed by any function scope, regardless of how many level up is that function scope
        // Cannot just check scope_depth == 0, because code might be in a scope but not necessarily in the scope of a function body
        if self.function_scopes == 0 {
            return Err(CompileError::ReturnOutsideFunction(
                self.parser.current.line,
            ));
        }

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

    /// An expression statement is an expression followed by a semicolon.
    /// They’re how you write an expression in a context where a statement is expected.
    /// Usually, it’s so that you can call a function or evaluate an assignment for its side effect.
    /// An expression statement evaluates the expression and discards the result from the stack.
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

    /* ============= End of Statement compiler methods ============= */

    /*
        ================ Expression compiler methods ================

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
            // @todo Add compile time error to ensure that the identifier must exist
            Err(_) => self.emit_code(OpCode::IDENTIFIER_LOOKUP(identifier)),
        };

        Ok(())
    }

    // @todo Handle calls with arguments
    /// Method to compile function calls
    pub fn call(&mut self) -> Result<(), CompileError> {
        // Immediately before 'call compiler method' is called, the opcode in front of it is assumed to hold info to get the function's identifier
        // Need to clone the string out to not hold onto self immutably
        let function_name = match self.chunk.codes.last() {
            // The identifier is stored directly for global scope identifier lookups
            Some(OpCode::IDENTIFIER_LOOKUP(identifier)) => identifier,

            // For locals, get the function name by looking into the locals vector in compiler
            Some(OpCode::GET_LOCAL(stack_index)) => &self.locals[*stack_index].name,

            _ => panic!("Compiler Debug Error: Unable to get function identifier from codes"),
        }
        .clone();

        // Get the number of arguments used for this function call
        let number_of_args: usize = if self.parser.check(TokenType::RightParen) {
            // If function call closed with no arguements, return 0
            0
        } else {
            let mut _number_of_args = 0;

            // Do while loop to compile the arguments
            loop {
                self.expression()?;
                _number_of_args += 1;

                if !self.parser.match_next(TokenType::Comma) {
                    break;
                }
            }

            _number_of_args
        };

        // Get the function parameters count stored in the hashmap
        let functions_parameter_count = self.functions_parameter_count.get(&function_name);

        // Runtime check on debug builds to ensure function parameter count is actually stored
        #[cfg(debug_assertions)]
        if functions_parameter_count.is_none() {
            panic!(format!(
                "Compiler Debug Error: '{}' function parameter count not stored",
                function_name
            ));
        }

        if *functions_parameter_count.unwrap() != number_of_args {
            return Err(CompileError::MismatchedArgumentCount(
                *functions_parameter_count.unwrap(),
                number_of_args,
            ));
        }

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

    /// The 2 operands of 'and' will be typed check to be bool.
    ///
    /// The first operand on the left hand side, is checked by JUMP_IF_FALSE opcode, which requires bool conditionals.
    ///
    /// The second operand on the right hand side, is checked by TYPE_CHECK_BOOL opcode, which throws runtime error if last value on stack is not bool.
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

    /// The 2 operands of 'or' will be typed check to be bool.
    ///
    /// The first operand on the left hand side, is checked by JUMP_IF_FALSE opcode, which requires bool conditionals.
    ///
    /// The second operand on the right hand side, is checked by TYPE_CHECK_BOOL opcode, which throws runtime error if last value on stack is not bool.
    ///
    /// @todo Because of how we compile this, if the first value is true, the second value will not be type checked
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

    /// Parse expression by using the TokenType to get a ParseRule's parse/compile method
    /// Continues to parse/compile infix operators if the precedence level is low enough
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
