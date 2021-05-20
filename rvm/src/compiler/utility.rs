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

        // Return code vec length after appending jump opcode, to use for calculating JUMP offset to back patch into the above instruction later
        self.chunk.codes.len() - 1
    }

    // Utility method to patch a JUMP type opcode with the actual jump offset value
    // JUMP instructions are first emitted with a 0 offset, before the offset is calculated and patched back in with this method
    pub fn patch_jump(&mut self, original_offset: usize) -> Result<(), CompileError> {
        let jump: usize = self.chunk.codes.len() - original_offset;

        // Create new opcode of the same JUMP type, but with the newly calculated jump offset value, and save it back into chunk
        self.chunk.codes[original_offset] = match &self.chunk.codes[original_offset] {
            OpCode::JUMP(0) => OpCode::JUMP(jump),
            OpCode::JUMP_IF_FALSE(0) => OpCode::JUMP_IF_FALSE(jump),
            invalid_opcode => return Err(CompileError::InvalidJumpOpcode(invalid_opcode.clone())),
        };
        Ok(())
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

    /// Utility method to generate POP type opcodes, to pop off all locals from stack that are no longer in scope
    pub fn pop_out_of_scope_locals_from_stack(&mut self) {
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

        if number_of_pops == 1 {
            // Use POP if there is exactly 1 local to pop off stack, as POP is more efficient than POP_N
            self.emit_code(OpCode::POP);
        } else if number_of_pops > 0 {
            // Use POP_N if there are more than 1 local to pop off the stack
            self.emit_code(OpCode::POP_N(number_of_pops));
        }
    }
}
