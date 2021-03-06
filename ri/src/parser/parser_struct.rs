use crate::token::Token;

pub struct Parser {
    // Expects ownership of token vector to be given
    pub tokens: Vec<Token>,
    pub current_index: usize, // current points at the current token
}
