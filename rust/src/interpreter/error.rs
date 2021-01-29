#[derive(Debug)]
pub enum RuntimeError {
    InternalError(String),
    TypeError(String),
    // UndefinedVariable(Token),
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
}

impl std::error::Error for RuntimeError {
    fn description(&self) -> &str {
        match *self {
            RuntimeError::InternalError(_) => "InternalError",
            RuntimeError::NegateNonNumberError(_) => "NegateNonNumberError",
            RuntimeError::SubtractNonNumbers(_) => "SubtractNonNumbers",
            RuntimeError::DivideNonNumbers(_) => "DivideNonNumbers",
            RuntimeError::MultiplyNonNumbers(_) => "MultiplyNonNumbers",
            RuntimeError::PlusTypeError(_) => "PlusTypeError",
            RuntimeError::GreaterNonNumbers(_) => "GreaterNonNumbers",
            RuntimeError::GreaterEqualNonNumbers(_) => "GreaterEqualNonNumbers",
            RuntimeError::LessNonNumbers(_) => "LessNonNumbers",
            RuntimeError::LessEqualNonNumbers(_) => "LessEqualNonNumbers",
            RuntimeError::DivideByZeroError(_) => "DivideByZeroError",
            RuntimeError::UndefinedVariable(_) => "UndefinedVariable",
            RuntimeError::CallOnNonCallable(_) => "CallOnNonCallable",
            RuntimeError::WrongArity(_, _, _) => "WrongArity",
            RuntimeError::InvalidGetTarget(_) => "InvalidGetTarget",
            RuntimeError::UndefinedProperty(_) => "UndefinedProperty",
            RuntimeError::InvalidSuperclass(_) => "InvalidSuperclass",
        }
    }
}
 */
