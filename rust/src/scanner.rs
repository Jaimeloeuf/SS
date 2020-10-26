// use crate::eat;
use crate::Keywords::KEYWORDS;
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
    // line: u32, // which line of the source we are currently on, used mainly for error reporting
    line: usize, // which line of the source we are currently on, used mainly for error reporting
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
            // self.scan_token();
            // self.add_token(TokenType::Str, value)
            match self.scan_token() {
                Some(token) => println!("{}", token.to_string()),
                None => println!("Not a token!"),
            }
        }

        // @todo Add Eof / Eol token
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        // self.tokens
        // println!("Logging out token vector");
        // for token in self.tokens.iter() {
        //     println!("{}", token.to_string())
        // }
        // println!("End of token vector");
    }

    fn get_token_type(&mut self, current_character: char) -> Option<TokenType> {
        match current_character {
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

                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
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
            '"' => Some(TokenType::Str),

            // Number Literals
            '0'..='9' => Some(TokenType::Number),

            // Alphabetic words
            // Identifiers, must start with alphabet or _, but can contain mix of alphanumeric chars
            'a'..='z' | 'A'..='Z' | '_' => Some(TokenType::Identifier),

            // Couldn't Match
            _ => {
                println!(
                    "Unexpected character '{}' on line {}",
                    current_character, self.line
                );
                // Call the error handling code

                None
            }
        }
    }

    // Construct token here then return option token
    fn scan_token(&mut self) -> Option<Token> {
        let current_character: char = self.advance();
        // println!("Current char {}", current_character);
        let token_type: Option<TokenType> = self.get_token_type(current_character);

        // Match current_character and maybe next character to a TokenType or None
        match token_type {
            Some(TokenType::Str) => {
                // let literal = eat::string(&mut source);
                // return Some(Token::new_string(&literal, *line));

                let literal = self.string_literals();
                Some(Token::new_string(literal.to_string(), self.line))
            }

            // All the other things that need more processing
            // '0'..='9' => {
            Some(TokenType::Number) => {
                // let literal = eat::number(&mut source);
                // return Some(Token::new_number(literal, *line));

                // Push current character back into source as advance methods removes it.
                // @todo Or maybe advance method shouldnt remove it? And just borrow ref here?
                self.source.push(current_character);
                let literal = self.number_literal();
                // Hmm this should not be a to_string
                Some(Token::new_number(literal.to_string(), self.line))
            }

            Some(TokenType::Identifier) => {
                self.source.push(current_character);
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

                let identifier = self.identifier();
                let keyword = KEYWORDS.get(&identifier);

                match keyword {
                    // If so, we use that keyword's token type.
                    Some(keyword) => Some(Token::new_keyword(keyword.clone(), self.line)),

                    // Otherwise, it's a regular user-defined identifier.
                    None => Some(Token::new_identifier(identifier, self.line)),
                }
            }

            Some(token_type) => {
                // Can unwrap here as we are sure that there is a value because of the Some wrap matching
                Some(Token::new_none_literal(
                    token_type,
                    current_character.to_string(),
                    self.line,
                ))
            }
            // Return None for Token back to caller of scan_token
            None => None,
        }
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

    // It's like a conditional advance(). We only consume the current character if it's what we're looking for.
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    // Returns the String literal between ""
    // @todo Should this be a new string or a slice?
    fn string_literals(&mut self) -> String {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            //   Lox.error(line, "Unterminated string.");
            println!("Unterminated string.");
            return "".to_string(); // Fix this... I need this to return smth, but shouldnt cos this should just error out
        }

        // The closing ".
        self.advance();

        // let value: String = self.source[self.start + 1..self.current - 1].to_string();
        // self.add_token(TokenType::Str, )
        // Trim the surrounding quotes.
        self.source[self.start + 1..self.current - 1].to_string()
    }

    // Returns the String literal between ""
    // @todo Should this be a new string or a slice?
    // Size of int is limited to isize? Should we support i128? Or how to do longs?
    fn number_literal(&mut self) -> isize {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        // Second part should be peek next
        if self.peek() == '.' && self.peek().is_ascii_digit() {
            // Consume fractional notation "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // If a none space character is chained to the number...
        // What about semicolon? Those are fine to chain right? Why dun we just ignore this and let the parser find issues?
        // if self.peek() != ' ' {
        // }

        // This should not be isize, as the value will be limited.
        // Perhaps we should save this as a string first, then only convert it later
        // @todo This will fail if it is a fraction
        self.source[self.start..self.current]
            .parse::<isize>()
            .unwrap()
    }

    // @todo Should this be a new string or a slice?
    fn identifier(&mut self) -> String {
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
        // The supported alphanumeric characters
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.source[self.start..self.current].to_string()
    }

    // addToken() is for output
    // It grabs the text of the current lexeme and creates a new token for it. We'll use the other overload to handle tokens with literal values soon.
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
