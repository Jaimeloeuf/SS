use crate::token::Token;

/// Parser struct data structure holding all the data related to the parser while parsing.
/// All integer fields are limited by the size of an unsigned integer of the target system.
pub struct Parser {
    /// `tokens` expects ownership of token vector to be given
    pub tokens: Vec<Token>,

    /// Integer index pointing at the current token being parsed
    pub current_index: usize,
}
