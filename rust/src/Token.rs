use crate::TokenType::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u32,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!(
            "{:?} {} {:?} {}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}

// @todo Tmp testing code
pub fn test() {
    let token = Token {
        token_type: TokenType::True,
        lexeme: String::from("TEST_LEXEME"),
        literal: None,
        line: 1,
    };
    println!("{}", token.to_string());
}
