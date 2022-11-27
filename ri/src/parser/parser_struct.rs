use crate::token::Token;

/// Parser struct data structure holding all the data related to the parser while parsing.
/// All integer fields are limited by the size of an unsigned integer of the target system.
///
/// This struct cannot outlive the reference to the Vector of Tokens used for parsing.
pub struct Parser<'lifetime_of_tokens> {
    /// `tokens` holds a immutable reference to the token vector created by the Scanner.
    pub tokens: &'lifetime_of_tokens Vec<Token>,

    /// Integer index pointing at the current token being parsed
    pub current_index: usize,
}
