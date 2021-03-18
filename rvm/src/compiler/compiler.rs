use super::parse_rule::{get_rule, ParseFn, ParseRule, Precedence, USIZE_TO_PRECEDENCE};

use crate::chunk::Chunk;
use crate::compiler::Parser;
use crate::opcode::OpCode;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token::TokenType;
use crate::value::Value;

pub struct Compiler {
    pub chunk: Chunk,

    // Hold a parser so that it can be passed along to the methods easily instead of relying on global state like clox
    pub parser: Parser,
}

impl Compiler {
    pub fn compile(source: String, chunk: Chunk) {
        let scanner = Scanner::new(source);

        // Create default token structs using the derived default trait, since at the start current and previous tokens does not exists yet
        let parser = Parser::new(scanner, Token::default(), Token::default());

        let mut compiler = Compiler { chunk, parser };

        // compiler.advance();
        // compiler.expression();
        // compiler.consume(TokenType::Eof, "Expect end of expression".to_string());
        compiler.parser.advance();
        compiler.expression();
        compiler
            .parser
            .consume(TokenType::Eof, "Expect end of expression".to_string());

        // self.advance();
        // self.expression();
        // self.consume(TokenType::Eof, "Expect end of expression".to_string());

        // Tmp add return code to use VM to print the return value
        compiler.emit_code(OpCode::RETURN);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let value: f64 = self.parser.scanner.source
            [self.parser.previous.start..self.parser.previous.length]
            .parse::<f64>()
            .unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser.consume(
            TokenType::RightParen,
            "Expect ')' after expression".to_string(),
        );
    }

    fn unary(&mut self) {
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
