use crate::opcode::OpCode;

#[derive(Debug)]
pub struct Chunk {
    pub codes: Vec<OpCode>,

    // Optimize storing line info: https://en.wikipedia.org/wiki/Run-length_encoding
    pub lines: Vec<usize>,
}

impl Chunk {
    // Associated function to create a new Chunk
    pub fn new() -> Chunk {
        Chunk {
            // codes: Vec::<OpCode>::new(),
            codes: Vec::<OpCode>::with_capacity(6),
            lines: Vec::new(),
        }
    }

    // Method to write a new OpCode
    pub fn write(&mut self, byte: OpCode, line_number: usize) {
        self.codes.push(byte);
        self.lines.push(line_number);
    }
}
