use crate::token::Token;

#[derive(Debug)]
pub enum ResolvingError {
    InternalError(&'static str),
    UndefinedIdentifier(Token),
    IdentifierAlreadyUsed(Token, String),
    IdentifierAlreadyUsedGlobally(Token, String),
    ReturnOutsideFunction(usize),

    /// EmptyBlockStatement(line_number_of_closing_right_brace)
    ///
    /// As a side effect, a "no-op" function cannot be defined in SS therefore if needed, it must be a native function
    EmptyBlockStatement(usize),

    /// UnreachableCode(error_message_on_the_cause_of_unreachable_code)
    ///
    /// Generic error for any unreachable code caused by any type of halting stmt
    UnreachableCode(String),
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
            ResolvingError::ReturnOutsideFunction(line_number) => write!(
                f,
                "[line {}] Cannot use `return` outside a function",
                line_number
            ),
            ResolvingError::UnreachableCode(message) => write!(f, "{}", message),
            ResolvingError::EmptyBlockStatement(line_number) => write!(
                f,
                "[line {}] Empty block statements are not allowed",
                line_number
            ),
        }
    }
}
