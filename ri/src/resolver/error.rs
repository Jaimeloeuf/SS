use crate::token::Token;

#[derive(Debug)]
pub enum ResolvingError {
    InternalError(&'static str),
    UndefinedIdentifier(Token),
    IdentifierAlreadyUsed(Token, String),
    IdentifierAlreadyUsedGlobally(Token, String),
    ReturnOutsideFunction(Token),
}

impl std::fmt::Display for ResolvingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolvingError::InternalError(ref message) => write!(f, "{}", message),
            ResolvingError::UndefinedIdentifier(ref token) => write!(
                f,
                "[line {}] Cannot access value of identifier '{}' before it is defined",
                token.line,
                token.lexeme.as_ref().unwrap()
            ),
            ResolvingError::IdentifierAlreadyUsed(ref token, ref identifier) => write!(
                f,
                "[line {}] Identifier '{}' cannot be reused, identifiers must be unique",
                token.line, identifier
            ),
            ResolvingError::IdentifierAlreadyUsedGlobally(ref token, ref identifier) => write!(
                f,
                "[line {}] Identifier '{}' is a Global SimpleScript identifier that cannot be reused",
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