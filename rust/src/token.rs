use crate::token_type::TokenType;

// If all the construction is done through the new method impls should
// we still make this pub? Or just make it pub so that people know what is this type but they shouldnt be able to use it?

// Debug trait to allow debug printing in the error handling code.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    // pub lexeme: String, // Ref or new string?
    pub literal: Option<String>,
    pub line: usize,
}

// For all the methods, should lexeme and line come first since they are always needed
impl Token {
    // Or should this be just new? then we assume that it is always a none literal
    pub fn new_none_literal(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            // lexeme,
            literal: None,
            line,
        }
    }

    // pub fn new_keyword(token_type: TokenType, line: usize) -> Token {
    pub fn new_keyword(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            // @todo I guess this should be wrapped in Option?
            // lexeme: "".to_string(), // Is this needed? Since the token type itself is always the keyword... why do we need to store the keyword again?
            literal: None,
            line,
        }
    }

    pub fn new_identifier(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Identifier,
            // lexeme,
            literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            line,
        }
    }

    pub fn new_string(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Str,
            // lexeme,
            literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            line,
        }
    }

    pub fn new_number(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            // lexeme,
            literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            line,
        }
    }

    // @todo Temporary allow this
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        // format!(
        //     "{:?} {} {:?} {}",
        //     self.token_type, self.lexeme, self.literal, self.line
        // )
        format!("{:?} {:?} {}", self.token_type, self.literal, self.line)
    }
}
