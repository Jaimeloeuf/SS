use crate::literal::Literal;
use crate::token_type::TokenType;

/// Debug trait to allow debug printing in the error handling code.
/// PartialEq trait is used to allow comparing of Token's in TypeChecker
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,

    // Only create and store literal for Literal values (String/Number/Bool/Null)
    pub literal: Option<Literal>,

    // Lexeme is stored as a string ONLY for identifier type tokens,
    // As this is reused very often and it is tedious to extract from Option<Literal::String> type every single time
    pub lexeme: Option<String>,
    pub line: usize,
}

// For all the methods, should lexeme and line come first since they are always needed
impl Token {
    pub fn new_none_literal(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            literal: None,
            lexeme: None,
            line,
        }
    }

    pub fn new_keyword(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            literal: None,
            lexeme: None,
            line,
        }
    }

    pub fn new_identifier(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: None,
            lexeme: Some(lexeme),
            line,
        }
    }

    pub fn new_string(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Str,
            literal: Some(Literal::String(lexeme)),
            lexeme: None,
            line,
        }
    }

    pub fn new_number(number: f64, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            // Treating all numbers as f64 type for now
            literal: Some(Literal::Number(number)),
            lexeme: None,
            line,
        }
    }

    // Only used for debugging token stream from scanner in debug builds
    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    pub fn to_debug_string(&self) -> String {
        if self.literal.is_none() {
            format!("[Line {}] {:?}", self.line, self.token_type)
        } else {
            format!(
                "[Line {}] {:?} -> {}",
                self.line,
                self.token_type,
                self.literal.clone().unwrap()
            )
        }
    }
}

// Implement the Display trait for Token directly, instead of using a to_string method on Token and requiring the caller to call it to print
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(ref literal) = self.literal {
            match literal {
                // Special way to print string literals of "String" token type
                Literal::String(ref string) => write!(f, "String '{}'", string),
                none_string => write!(f, "{:?} {}", self.token_type, none_string),
            }
        } else {
            // If there is a lexeme, it means this token is an identifier
            if let Some(ref lexeme) = self.lexeme {
                write!(f, "{:?} - {}", self.token_type, lexeme)
            } else {
                write!(f, "{:?}", self.token_type)
            }
        }
    }
}
