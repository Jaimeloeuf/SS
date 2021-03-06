use crate::token::Token;

#[derive(Debug)]
pub enum ResolvingError {
    UndefinedIdentifier(Token),
    IdentifierAlreadyUsed(Token, String),
    ReturnOutsideFunction(Token),
}

impl std::fmt::Display for ResolvingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolvingError::UndefinedIdentifier(ref token) => write!(
                f,
                "[line {}] Cannot access value of identifier {} before it is defined",
                token.line,
                token.literal.as_ref().unwrap()
            ),
            ResolvingError::IdentifierAlreadyUsed(ref token, ref identifier) => write!(
                f,
                "[line {}] Identifier '{}' cannot be reused, identifiers must be unique",
                token.line, identifier
            ),
            ResolvingError::ReturnOutsideFunction(ref token) => write!(
                f,
                "[line {}] Cannot use `return` outside a function",
                token.line
            ),
        }
    }
}
