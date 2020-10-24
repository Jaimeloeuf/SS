use crate::TokenType::TokenType;

struct Token {
    token_type: TokenType,
    lexeme: String,
    // literal: Object,
    literal: bool,
    line: u32,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "{:?} {} {} {}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}

// @todo Tmp testing code
pub fn test() {
    let token = Token {
        token_type: TokenType::True,
        lexeme: String::from("TEST_LEXEME"),
        literal: true,
        line: 1,
    };
    println!("{}", token.to_string());
}
