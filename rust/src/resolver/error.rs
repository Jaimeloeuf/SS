use crate::token::Token;

#[derive(Debug)]
pub enum ResolvingError {
    IdentifierAlreadyUsed(Token, String),
    ToplevelReturn(Token),
}

impl std::fmt::Display for ResolvingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolvingError::IdentifierAlreadyUsed(ref token, ref identifier) => write!(
                f,
                "[line {}] Identifier '{}' cannot be reused, identifiers must be unique",
                token.line, identifier
            ),
            ResolvingError::ToplevelReturn(ref token) => write!(
                f,
                "[line {}] Cannot use `return` outside a function",
                token.line
            ),
        }
    }
}
