use crate::token::Token;

/**
 * Enum of all possible Runtime Errors
 * String is used instead of &str, as some of the strings are formatted on the fly using format!()
 *
 * @todo Should line number be included here for logging?
 * @todo Remove debug trait once Display trait is implemented
 */
#[derive(Debug)]
pub enum RuntimeError {
    InternalError(String),
    // @todo Maybe store, given type, and expected type
    // @todo String or &str?
    TypeError(String),
    UndefinedVariable(String),
    // NegateNonNumberError(Token),
    // SubtractNonNumbers(Token),
    // DivideNonNumbers(Token),
    // MultiplyNonNumbers(Token),
    // PlusTypeError(Token),
    // GreaterNonNumbers(Token),
    // GreaterEqualNonNumbers(Token),
    // LessNonNumbers(Token),
    // LessEqualNonNumbers(Token),
    // DivideByZeroError(Token),
    // CallOnNonCallable(Token),
    // WrongArity(Token, usize, usize),
    // InvalidGetTarget(Token),
    // UndefinedProperty(Token),
    // InvalidSuperclass(Token),
}

// @todo Code copied from rlox, to be changed later
/* impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RuntimeError::InternalError(ref message) => {
                write!(f, "Internal interpreter error: {}", message)
            }
            RuntimeError::NegateNonNumberError(ref token) => write!(
                f,
                "[line {}] Cannot negate a non-numerical value",
                token.line
            ),
            RuntimeError::SubtractNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a subtraction must be numbers",
                token.line
            ),
            RuntimeError::DivideNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a division must be numbers",
                token.line
            ),
            RuntimeError::MultiplyNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a multiplication must be numbers",
                token.line
            ),
            RuntimeError::PlusTypeError(ref token) => write!(
                f,
                "[line {}] Both sides of an addition must be either strings or numbers",
                token.line
            ),
            RuntimeError::GreaterNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a greater than comparison must be numbers",
                token.line
            ),
            RuntimeError::GreaterEqualNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a greater or equal comparison must be numbers",
                token.line
            ),
            RuntimeError::LessNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a less than comparison must be numbers",
                token.line
            ),
            RuntimeError::LessEqualNonNumbers(ref token) => write!(
                f,
                "[line {}] Both sides of a less or equal comparison must be numbers",
                token.line
            ),
            RuntimeError::DivideByZeroError(ref token) => {
                write!(f, "[line {}] Cannot divide by zero", token.line)
            }
            RuntimeError::UndefinedVariable(ref token) => write!(
                f,
                "[line {}] Undefined variable `{}`",
                token.line, token.lexeme
            ),
            RuntimeError::CallOnNonCallable(ref token) => {
                write!(f, "[line {}] Attempted to call on non-callable", token.line)
            }
            RuntimeError::WrongArity(ref token, actual, expected) => write!(
                f,
                "[line {}] Function arity error, expected {} arguments but got {}",
                token.line, expected, actual
            ),
            RuntimeError::InvalidGetTarget(ref token) => write!(
                f,
                "[line {}] Only instances have properties, tried to access `{}` in non-instance",
                token.line, token.lexeme
            ),
            RuntimeError::UndefinedProperty(ref token) => write!(
                f,
                "[line {}] Undefined property `{}`.",
                token.line, token.lexeme
            ),
            RuntimeError::InvalidSuperclass(ref token) => write!(
                f,
                "[line {}] Invalid parent class for `{}`.",
                token.line, token.lexeme
            ),
        }
    }
}*/
