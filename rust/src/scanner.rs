// use crate::eat;
use crate::Token::Token;
use crate::TokenType::TokenType;

#[allow(dead_code)]
pub struct Scanner {
    // @todo Should this be static lifetime?
    // source: &'static mut String,
    source: String,
    // tokens: vec!
    tokens: Vec<Token>,

    // usize for fn is_at_end -> bool cos the source.len is of type usize
    start: usize, // start field points to the first character in the lexeme being scanned
    current: usize, // current points at the character currently being considered

    // The line field tracks what source line current is on so we can produce tokens that know their location.
    line: u32, // which line of the source we are currently on, used mainly for error reporting
}

impl Scanner {
    // Constructor
    // Should this be a mutable reference or just give this ownership
    // pub fn new(source: String) -> Scanner {
    // pub fn new(source: &'static mut std::string::String) -> Scanner {
    pub fn new(source: &mut String) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // pub fn scan_tokens(&mut self) -> Vec<Token> {
    pub fn scan_tokens(&mut self) {
        // pub fn scan_tokens(&mut self) {
        // Each turn of the loop, we scan a single token.
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // @todo Add Eof / Eol token
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        // self.tokens
        for token in self.tokens.iter() {
            println!("{}", token.to_string())
        }
    }

    fn scan_token(&mut self) {
        let current_character: char = self.advance();

        let token_type = match current_character {
            // Standard tokens
            ';' => Some(TokenType::Semicolon),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),

            // Math operators
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '*' => Some(TokenType::Star),

            // For lexeme that can be "chained" / have another char behind it to form a lexeme of 2 chars
            '!' if self.conditional_advance('=') => Some(TokenType::BangEqual),
            '!' => Some(TokenType::Bang),
            '=' if self.conditional_advance('=') => Some(TokenType::EqualEqual),
            '=' => Some(TokenType::Equal),
            '<' if self.conditional_advance('=') => Some(TokenType::LessEqual),
            '<' => Some(TokenType::Less),
            '>' if self.conditional_advance('=') => Some(TokenType::GreaterEqual),
            '>' => Some(TokenType::Greater),

            // |

            /* Inline and Block Comment */
            // A comment goes until the end of the line.
            '/' if self.conditional_advance('/') => {
                // eat::line(&mut source);
                // // return None;
                // return ();

                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }

                return ();
            }
            // @todo To support block comments once eat methods are completed
            // '/' if self.conditional_advance('*') => {
            //     eat::block_comment(&mut source);
            //     // return None;
            //     return ();
            // }
            '/' => Some(TokenType::Slash),

            // Whitespace input types
            ' ' => None,
            '\r' => None,
            '\t' => None,

            '\n' => {
                self.line += 1;
                None
            }

            // String Literals
            // '"' => {
            //     let literal = eat::string(&mut source);
            //     return Some(Token::new_string(&literal, *line));
            // }
            // Number Literals
            // '0'..='9' => {
            //     source.push(c);
            //     let literal = eat::number(&mut source);
            //     return Some(Token::new_number(literal, *line));
            // }
            // Alphabetic words
            // 'a'...'z' | 'A'...'Z' => {
            // 'a'..='z' | 'A'..='Z' => {
            // source.push(c);
            // let lexeme = eat::identifier(&mut source);
            // match KEYWORDS.get(&lexeme) {
            //     Some(type_of) => {
            //         let type_of = *type_of;
            //         return Some(Token::new_keyword(type_of, *line));
            //     }
            //     None => {
            //         return Some(Token::new_identifier(&lexeme, *line));
            //     }
            // }
            // }

            // Couldn't Match
            _ => {
                println!(
                    "Unexpected character '{}' on line {}",
                    current_character, self.line
                );
                // Call the error handling code

                return ();
            }
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // advance() is for input
    // Consume next character from source and return it.
    fn advance(&mut self) -> char {
        self.current += 1;
        // self.source.charAt(current - 1)
        // self.source.get((self.current - 1)..self.current)
        // let ch = self.source[self.current - 1];
        self.source.chars().nth(self.current - 1).unwrap()
    }

    // It’s like a conditional advance(). We only consume the current character if it’s what we’re looking for.
    fn conditional_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || (self.source.chars().nth(self.current).unwrap() != expected) {
            return false;
        }

        // Advance if the expected character is found
        self.current += 1;

        // Return true
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    // addToken() is for output
    // It grabs the text of the current lexeme and creates a new token for it. We’ll use the other overload to handle tokens with literal values soon.
    // Add basic token is just add token but without any literal
    fn add_basic_token(&mut self, token_type: TokenType) {
        // Perhaps use something like None instead?
        self.add_token(token_type, "".to_string());
    }

    // @todo Fix the literal type
    fn add_token(&mut self, token_type: TokenType, literal: String) {
        // let text: String = self.source[self.start..self.current];
        let lexeme = String::from("tmp");

        // self.tokens.push(Token {
        //     token_type,
        //     lexeme,
        //     literal: None,
        //     line: self.line,
        // });
    }
}
