use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    // Expects ownership of token vector to be given
    tokens: Vec<Token>,
    current: usize, // current points at the current token
}

pub struct Stmt {
    //
}

pub struct Expr {
    //
}

impl Expr {
    //
}

// Infrastructure/Utility methods on the Parser struct
impl Parser {
    // Returns immutable reference to the current token
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.check(TokenType::Eof)
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> &Token {
        // Only increment the current token counter if not at end yet
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    // checks if current token has any of the given types.
    // If so, consumes the token and returns true.
    // Otherwise, returns false and leave current token alone
    fn is_next_token(&mut self, token_types_to_check: Vec<TokenType>) -> bool {
        for token_type in token_types_to_check {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }
}

impl Parser {
    // Constructor
    // Takes ownership of the token vector
    pub fn new(tokens: Vec<Token>) -> Parser {
        println!("Processing '{}' tokens", tokens.len());
        Parser { tokens, current: 0 }
    }

    // Statements and State parse-declaration statements.add(statement());
    // Moves a statments vector out. Move instead of borrow as vec created in this scope
    pub fn parse(&mut self) -> Vec<Stmt> {
        let statements: Vec<Stmt> = Vec::<Stmt>::new();
        // let statements: Vec<Stmt> = Vec::new();

        // On each loop, we scan a single token.
        while !self.is_at_end() {
            self.advance();
        }

        // Pass back immutable reference of the tokens vector
        statements
    }
}
