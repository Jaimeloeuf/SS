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
        // Compile the operand.
        self.expression();

        // Emit the operator instruction.
        match &self.parser.previous.token_type {
            TokenType::Minus => self.emit_code(OpCode::NEGATE),

            // Unreachable.
            _ => return,
        }
    }
}
