use super::parse_rule::{get_rule, ParseFn, ParseRule, Precedence, USIZE_TO_PRECEDENCE};

use crate::chunk::Chunk;
use crate::compiler::Parser;
use crate::opcode::OpCode;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;
use crate::value::Value;

// The Compiler / Parser / Scanner structs are strung together,
// Compiler struct holds a Parser
// Parser struct holds a Scanner
// Scanner is created inside compile method, it is used to create Parser struct, which is used to create the Compiler struct
pub struct Compiler {
    pub chunk: Chunk,

    // Hold a parser so that it can be passed along to the methods easily instead of relying on global state like clox
    pub parser: Parser,
}

impl Compiler {
    // Returns Chunk that can be run immediately
    pub fn compile(source: String, chunk: Chunk) -> Chunk {
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
        };

        // compiler.advance();
        // compiler.expression();
        // compiler.consume(TokenType::Eof, "Expect end of expression".to_string());
        compiler.parser.advance();
        compiler.expression();
        compiler
            .parser
            .consume(TokenType::Eof, "Expect end of expression".to_string());

        // @todo Tmp add return code to use VM to print the return value
        compiler.emit_code(OpCode::RETURN);

        // Return the chunk, now that it is filled with OpCodes from Compiler struct
        compiler.chunk
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn number(&mut self) {
        let value: f64 = self.parser.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length]
            .parse::<f64>()
            .unwrap();
        self.emit_constant(Value::Number(value));
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after expression".to_string(),
        );
    }

    pub fn unary(&mut self) {
        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match &self.parser.previous.token_type {
            TokenType::Minus => self.emit_code(OpCode::NEGATE),

            // Unreachable
            // @todo Change to use Result error variant to bubble error up
            _ => return,
        }
    }


    // Parse expression by using the TokenType to get a ParseRule's parse/compile method
    // Continues to parse/compile infix operators if the precedence level is low enough
    fn parse_precedence(&mut self, precedence: Precedence) {
        // Shadow precedence variable to convert it from enum variant to usize for numerical comparison later
        let precedence = precedence as usize;

        // Read the next token
        self.parser.advance();

        // Look up corresponding ParseRule of the previous token's TokenType, and match to use the prefix parser
        match get_rule(&self.parser.previous.token_type).prefix {
            // Alternative syntax for self.prefix_rule() where prefix_rule is a variable function pointer
            Some(prefix_rule) => prefix_rule(self),

            // If there is no prefix parser, then the token must be a syntax error
            // @todo Handle error using an Result error variant
            None => return eprintln!("Expect expression. No prefix parser"),
        };

        // After parsing the prefix expression, which may consume more tokens this look for an infix parser for the next token.
        // If there is one, it means the prefix expression this just compiled might be an operand for it,
        // BUT ONLY if the call to parse_precedence() has a precedence that is low enough to permit that infix operator.
        // To test if it is low enough, convert ParseRule's precedence into its usize discriminant to compare with the precedence passed in
        while precedence <= get_rule(&self.parser.current.token_type).precedence as usize {
            // Read the next token
            self.parser.advance();

            // Look up corresponding ParseRule of the previous token's TokenType, and match to use the infix parser
            match get_rule(&self.parser.previous.token_type).infix {
                // Alternative syntax for self.infix_rule() where infix_rule is a variable function pointer
                Some(infix_rule) => infix_rule(self),

                // If there is no prefix parser, then the token must be a syntax error
                // @todo Handle error using an Result error variant
                None => return eprintln!("Expect expression. No infix parser"),
            }
        }
    }
}
