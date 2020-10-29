use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    // @todo Maybe remove static
    tokens: &'static Vec<Token>,

    // usize for fn is_at_end -> bool cos the source.len is of type usize
    current: usize, // current points at the character currently being considered
}

struct Stmt {
    //
}

struct Expr {
    //
}

impl Expr {
    //
}

impl Parser {
    // Constructor
    // Should this be a mutable reference or just give this ownership
    pub fn new(tokens: &'static Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // Statements and State parse-declaration statements.add(statement());
    // Moves a statments vector out. Move instead of borrow as vec created in this scope
    pub fn parse(&mut self) -> Vec<Stmt> {
        let statements: Vec<Stmt> = Vec::<Stmt>::new();
        // let statements: Vec<Stmt> = Vec::new();

        // On each loop, we scan a single token.
        while !self.is_at_end() {
            // statements.push(self.declaration());
            let declaration = self.declaration();
            statements.push(declaration);
        }

        // Pass back immutable reference of the tokens vector
        statements
    }

    fn is_at_end(&self) -> bool {
        // self.peek().token_type == TokenType::Eof
        // @todo Kinda dumb, why cant I just do simple equality
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    // Get next character in source string without advancing index of current character
    // Used to check lexical grammar
    // Get back immutable reference to Token instead.
    fn peek(&self) -> &Token {
        // not using get as it returns an option instead, and not sure how to handle None...
        self.tokens.get(self.current).unwrap()
        // Will cause program to panic if the index is invalid
        // file:///C:/Users/JJ/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/ch08-01-vectors.html#reading-elements-of-vectors
        // &self.tokens[self.current]
    }

    // Get next next character in source string without advancing index of current character
    // Used to check lexical grammar
    fn peek_next(&mut self) -> Option<&Token> {
        // if self.current + 1 >= self.source.len() {
        if self.current + 1 >= self.tokens.len() {
            // '\0'
            None
        } else {
            Some(self.tokens.get(self.current + 1).unwrap())
        }
    }
}
