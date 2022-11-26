/// Scanner struct data structure holding all the data related to the scanner while scanning.
/// All integer fields are limited by the size of an unsigned integer of the target system.
pub struct Scanner {
    pub source: String,

    /// `start` field points to the first character in the lexeme being scanned.
    /// This is usize for `fn is_at_end -> bool` because source.len is of type usize
    pub start: usize,

    /// `current` points at the character currently being considered
    pub current: usize,

    /// This tracks the line scanner is currently on in the source file to produce tokens that know their location and for error reporting
    pub line: usize,
}
