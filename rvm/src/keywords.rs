use crate::token::TokenType;

// Function to return corresponding TokenType if input string is a keyword
pub fn get_token_type_if_keyword(word_to_test_that_might_be_keyword: &str) -> Option<TokenType> {
    match word_to_test_that_might_be_keyword {
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),

        // To implement?
        // "switch" => Some(TokenType::Break),
        // "break" => Some(TokenType::Break),

        // Loops...
        // "for" => Some(TokenType::For),
        "while" => Some(TokenType::While),

        // Function related stuff
        // "async" => Some(TokenType::Async),
        // "pure" => Some(TokenType::Pure),
        "function" => Some(TokenType::Function),
        "return" => Some(TokenType::Return),

        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),

        // @todo Should these be included...
        "print" => Some(TokenType::Print),
        "null" => Some(TokenType::Null),

        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),

        "const" => Some(TokenType::Const),

        _ => None,
    }
}
