use super::Compiler;

use crate::opcode::OpCode;
use crate::token::Token;
use crate::token::TokenType;

impl Compiler {
    fn error_at_current(message: String) {
        //
    }

    fn error_at(token: &Token, message: String) {
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            // eprint!("{:0width$}", token.start, width = token.length);
            eprint!(" at '{}'", token.start);
        }

        eprintln!(": {}", message);
        // parser.hadError = true;
    }

    fn emit_code(&mut self, code: OpCode) {
        self.chunk.write(code, parser.previous.line);
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if (parser.current.token_type == token_type) {
            return self.advance();
        }

        self.error_at_current(message);
    }
}
