use crate::literal::Literal;
use crate::token_type::TokenType;

// Debug trait to allow debug printing in the error handling code.
// Tmp add clone trait for parser utiltiy_traits' "consume" method
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    // pub lexeme: String, // Ref or new string?
    pub literal: Option<Literal>,
    pub line: usize,
}

// For all the methods, should lexeme and line come first since they are always needed
impl Token {
    pub fn new_none_literal(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            literal: None,
            line,
        }
    }

    pub fn new_keyword(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            literal: None,
            line,
        }
    }

    pub fn new_identifier(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: Some(Literal::String(lexeme)),
            line,
        }
    }

    pub fn new_string(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Str,
            literal: Some(Literal::String(lexeme)),
            line,
        }
    }

    pub fn new_number(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            // Treating all numbers as f64 type for now
            literal: Some(Literal::Number(lexeme.parse::<f64>().unwrap())),
            line,
        }
    }

    // Only used for debugging token stream from scanner
    #[allow(dead_code)]
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
        if self.literal.is_none() {
            write!(f, "{:?}", self.token_type)
        } else {
            match self.literal.as_ref().unwrap() {
                // Special way to print string literals belong to "String" token type and not a Identifier
                Literal::String(ref string) if self.token_type != TokenType::Identifier => {
                    write!(f, "String '{}'", string)
                }
                none_string => write!(f, "{:?} {}", self.token_type, none_string),
            }
        }
    }
}
