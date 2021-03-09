/**
 * WIP Scaffolded from rust interpreter.
 *
 * Enum of all possible Runtime Errors
 * String is used instead of &str, as some of the strings are formatted on the fly using format!()
 *
 * @todo Should line number be included here for logging?
 * @todo Remove debug trait once Display trait is implemented
 */
#[derive(Debug)]
pub enum RuntimeError {
    // Split it up, add a internal parsing error
    // Internal error should be like generic unrepeating errors
    // But alot of checks are for internal errors caused by parser
    InternalError(String),

    // @todo Maybe store, given type, and expected type
    // @todo String or &str?
    // Cast error?
    // Add type found, by passing in the Value type?
    TypeError(String),

    // Basically a specific type of TypeError, where a bool is expected for a condition
    // Conditions can be If conditionals to loop continuation conditions
    ConditionTypeError(String),

    // Undefined values and variables, 1 for const and 1 for variables
    // @todo Undefined variable will not be used since it will always be parsed as Expr::Const for now, thus always UndefinedIdentifier
    UndefinedIdentifier(usize, String),
    UndefinedVariable(String),

    // @todo Should be a SyntaxError or ParsingError instead, basically should not be RuntimeError as this error should be found before runtime
    // When a Const has already been defined in the current environment/scope a new one should not be allowed.
    ValueAlreadyDefined(String),

    // @todo Include line info somehow
    ArrayOutOfBounds(String),

    // Tried using a none callable Value type as a function identifier and calling it as a function
    // usize holds the line number of the call site
    // String is the string representation of Value object that the user tried to call
    CallOnNonCallable(usize, String),
    // DivideByZeroError(Token),
    // WrongArity(Token, usize, usize),
    // UndefinedProperty(Token),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // @todo Perhaps wrap this, and below add stack trace or at least file and line number where error occurred
        match self {
            RuntimeError::InternalError(ref message) => {
                write!(f, "Internal interpreter error: {}", message)
            }

            RuntimeError::TypeError(ref message) => write!(f, "Type Error: {}", message),
            RuntimeError::ConditionTypeError(ref message) => write!(f, "{}", message),

            // @todo DELETE THIS, should be caught by resolver already?? Would there be cases where this is not caught?
            RuntimeError::UndefinedIdentifier(ref line_number,identifier) => {
                write!(f, "[line {}] ReferenceError: Tried to use undefined identifier '{}'", line_number, identifier)
            }

            RuntimeError::ValueAlreadyDefined(ref identifier) => {
                write!(f, "ReferenceError: Identifier '{}' already used in current scope!", identifier)
            }

            RuntimeError::ArrayOutOfBounds(ref message) => write!(f, "{}", message),

            RuntimeError::CallOnNonCallable(ref line_number, ref value) => {
                write!(f, "[line {}] Attempted to call non-callable: {}", line_number, value)
            }

            // If unimplemented yet print with debug symbol to prevent infinite recursive loop to calling the display trait
            runtime_error_variant => write!(f, "Internal error with unimplemented formatting:\n{:?}", runtime_error_variant)
            // RuntimeError::UndefinedVariable(ref token) => write!(
            //     f,
            //     "[line {}] Undefined variable `{}`",
            //     token.line, token.lexeme
            // ),
            // RuntimeError::WrongArity(ref token, actual, expected) => write!(
            //     f,
            //     "[line {}] Function arity error, expected {} arguments but got {}",
            //     token.line, expected, actual
            // ),
            // RuntimeError::UndefinedProperty(ref token) => write!(
            //     f,
            //     "[line {}] Undefined property `{}`.",
            //     token.line, token.lexeme
            // ),
        }
    }
}
