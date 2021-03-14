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

// Precedence enum where all these can be converted to usize
enum Precedence {
    NONE,
    ASSIGNMENT, // =
    OR,         // or
    AND,        // and
    EQUALITY,   // == !=
    COMPARISON, // < > <= >=
    TERM,       // + -
    FACTOR,     // * /
    UNARY,      // ! -
    CALL,       // . ()
    PRIMARY,
}

impl Compiler {
    pub fn compile(source: String, chunk: Chunk) {
        let compiler = Compiler { chunk };
    }
}

struct Parser {
    current: Token,
    previous: Token,
}

fn advance(scanner: &Scanner) {
    let mut parser = Parser {
        current: scanner.make_token(TokenType::Eof),
        previous: scanner.make_token(TokenType::Eof),
    };

    parser.previous = parser.current;

    loop {
        // parser.current = Scanner::scan_token();
        let current = *scanner::scan_token();
        if parser.current.token_type != TokenType::Error {
            break;
        } else {
            errorAtCurrent(parser.current.start);
        }
    }
}
