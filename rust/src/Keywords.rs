use crate::TokenType::TokenType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        // Should use
        // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.with_capacity
        let mut m = HashMap::new();

        // Might not want these
        m.insert("and".to_string(), TokenType::And);
        m.insert("or".to_string(), TokenType::Or);

        // Gotta implement this
        m.insert("switch".to_string(), TokenType::Break);
        m.insert("break".to_string(), TokenType::Break);

        // Loops...
        m.insert("for".to_string(), TokenType::For);
        m.insert("while".to_string(), TokenType::While);

        // Function related stuff
        m.insert("function".to_string(), TokenType::Function);
        m.insert("return".to_string(), TokenType::Return);
        // m.insert("async".to_string(), TokenType::Async);
        // m.insert("pure".to_string(), TokenType::Pure);

        m.insert("if".to_string(), TokenType::If);
        m.insert("else".to_string(), TokenType::Else);

        // Should these be included...
        m.insert("print".to_string(), TokenType::Print);
        m.insert("nil".to_string(), TokenType::Nil);

        m.insert("true".to_string(), TokenType::True);
        m.insert("false".to_string(), TokenType::False);

        m.insert("const".to_string(), TokenType::Const);

        m
    };
}

// // Use a function and match to return instead of a hashmap....
// // maybe not such a good idea
// pub fn get_token_type_from_keyword(
//     word_to_test_that_might_be_keyword: String,
// ) -> Option<TokenType> {
//     match word_to_test_that_might_be_keyword.as_str() {
//         // Might not want these
//         "and" => Some(TokenType::And),
//         "or" => Some(TokenType::Or),

//         // Gotta implement this
//         "switch" => Some(TokenType::Break),
//         "break" => Some(TokenType::Break),

//         // Loops...
//         "for" => Some(TokenType::For),
//         "while" => Some(TokenType::While),

//         // Function related stuff
//         "function" => Some(TokenType::Function),
//         "return" => Some(TokenType::Return),
//         // "async" => Some(TokenType::Async),
//         // "pure" => Some(TokenType::Pure),
//         "if" => Some(TokenType::If),
//         "else" => Some(TokenType::Else),

//         // Should these be included...
//         "print" => Some(TokenType::Print),
//         "nil" => Some(TokenType::Nil),

//         "true" => Some(TokenType::True),
//         "false" => Some(TokenType::False),

//         "const" => Some(TokenType::Const),

//         _ => None,
//     }
// }
