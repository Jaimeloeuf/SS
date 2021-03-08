// There are 2 types of OpCodes
// First type are the instruction OpCodes, that are denoted by their all CAPS spelling
// Secondly there are special OpCodes, that are variants with additional values like 'ConstantIndex(usize)'
//
// Clox differentiate OpCode from Data using OP_ prefix, and also all their data types is just a single byte anyways

#[derive(Debug)]
pub enum OpCode {
    RETURN,
    CONSTANT,

    ConstantIndex(usize),
}
