use crate::parser::stmt::Stmt;
use crate::token::Token;

#[derive(Debug)]
pub enum ResolvingError {
    InternalError(&'static str),
    UndefinedIdentifier(Token),
    IdentifierAlreadyUsed(Token, String),
    IdentifierAlreadyUsedGlobally(Token, String),
    ReturnOutsideFunction(usize),

    /// UnreachableCodeAfterReturn(line_number)
    ///
    /// There should not be any unreachable code after a return statement
    UnreachableCodeAfterReturn(usize),
    /// UnreachableCode(halting_stmt)
    ///
    /// A more generic error for any unreachable code
    UnreachableCode(Stmt),
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
            ResolvingError::UnreachableCodeAfterReturn(line_number) => write!(
                f,
                "[line {}] Unreachable code found after the `return` statement on this line",
                line_number
            ),
            ResolvingError::UnreachableCode(ref stmt) => write!(
                f,
                // Only need to handle Block / if / while statements for unreachable code
                "{}", match stmt {
                    Stmt::Block(_, Some(line_number)) => format!(
                        "[line {}] Unreachable code found after this line",
                        line_number
                    ),
                    Stmt::If(_, _, _, line_number) => format!(
                        "[line {}] Unreachable code found after this if-else statement",
                        line_number
                    ),
                    Stmt::While(_, _, line_number) => format!(
                        "[line {}] Unreachable code found after this while loop",
                        line_number
                    ),
                    // All other statement types cannot be halting, thus they will not appear here
                    _ => panic!("Invalid 'unreachable' statement: {:#?}", stmt),
                }
            ),
        }
    }
}
