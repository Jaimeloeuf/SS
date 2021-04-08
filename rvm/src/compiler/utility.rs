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

    // Utility method to emit a JUMP type opcode and return the current code position
    pub fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_code(instruction);

        // Return current code position, which is simply the last index as the code just emitted will be appended to the vec
        self.chunk.codes.len() - 1
    }

    // Utility method to patch a JUMP type opcode with the actual jump offset value
    // JUMP instructions are first emitted with a 0 offset, before the offset is calculated and patched back in with this method
    pub fn patch_jump(&mut self, offset: usize) -> Result<(), CompileError> {
        let jump: usize = self.chunk.codes.len() - offset - 1;

        Ok(match &self.chunk.codes[offset] {
            OpCode::JUMP(0) => self.chunk.codes[offset] = OpCode::JUMP(jump),
            OpCode::JUMP_IF_FALSE(0) => self.chunk.codes[offset] = OpCode::JUMP_IF_FALSE(jump),

            invalid_opcode => return Err(CompileError::InvalidJumpOpcode(invalid_opcode.clone())),
        })
    }

    // Indirection for all declaration and statement methods, to advance parser before calling the method
    // Inlined to remove runtime method call overhead
    #[inline]
    pub fn advance_and_call(
        &mut self,
        method: fn(&mut Compiler) -> Result<(), CompileError>,
    ) -> Result<(), CompileError> {
        // @todo Handle result variant
        self.parser.advance();
        method(self)
    }
}
