use super::CompileError;
use super::Compiler;

use crate::opcode::OpCode;
use crate::value::Value;

impl Compiler {
    // This should be under compiler instead
    pub fn emit_code(&mut self, code: OpCode) {
        self.chunk.write(code, self.parser.previous.line);
    }

    pub fn emit_constant(&mut self, value: Value) {
        self.emit_code(OpCode::CONSTANT(value));
    }

    // Indirection for all declaration and statement methods, to advance parser before calling the method
    // Inlined to remove runtime method call overhead
    // @todo compiler method and this method should return  -> Result<Stmt, ParsingError>
    #[inline]
    pub fn advance_and_call(&mut self, method: fn(&mut Compiler)) {
        self.parser.advance();
        method(self)
    }
}
