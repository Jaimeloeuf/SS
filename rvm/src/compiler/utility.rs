use super::Compiler;

use crate::opcode::OpCode;
use crate::value::Value;

impl Compiler {
    // This should be under compiler instead
    pub fn emit_code(&mut self, code: OpCode) {
        self.chunk.write(code, self.parser.previous.line);
    }

    pub fn emit_constant(&self, value: Value) {
        self.emit_code(OpCode::CONSTANT(value));
    }
}
