/*
Testing the use of Literal enum for the literal value of Token struct,
instead of just an optional string.

This forces us to deal with the type of the input chars/string before actual processing.
Might make it eaiser, but will see...

The other option is to keep the optional string, but store it as the lexeme value,
and use Literal enum variants for the literal value in token struct.
*/

use crate::literal::Literal;
use crate::token_type::TokenType;

// If all the construction is done through the new method impls should
// we still make this pub? Or just make it pub so that people know what is this type but they shouldnt be able to use it?

// Debug trait to allow debug printing in the error handling code.
// Tmp add clone trait for parser utiltiy_traits' "consume" method
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    // pub lexeme: String, // Ref or new string?
    // pub literal: Option<String>,
    pub literal: Option<Literal>,
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

    pub fn new_keyword(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            // Is this needed? Since the token type itself is always the keyword... why do we need to store the keyword again?
            // lexeme: "".to_string(),
            literal: None,
            line,
        }
    }

    pub fn new_identifier(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Identifier,
            // lexeme,
            // literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            literal: Some(Literal::String(lexeme)),
            line,
        }
    }

    pub fn new_string(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Str,
            // lexeme,
            // literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            literal: Some(Literal::String(lexeme)),
            line,
        }
    }

    pub fn new_number(lexeme: String, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            // lexeme,
            // literal: Some(lexeme), // ? Should this be like that? Is lexeme same as the string?
            // Although the input is a string, should this be parsed into a Number?
            literal: Some(Literal::String(lexeme)),
            line,
        }
    }

    // @todo Temporary allow this
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{} {:?} {:?}", self.line, self.token_type, self.literal)
    }
}
