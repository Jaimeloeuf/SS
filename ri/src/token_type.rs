/// Enum for all possible Token Types.
///
/// Inherited traits:
/// Debug is to make debug printing easier and skip implementing the display trait
/// PartialEq is used to do comparison instead of match. Refer to Parser struct methods
/// @todo Clone trait is needed as Token struct contains this, and Token struct derives the Clone trait
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    Semicolon,    // ;
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,
    Dot,

    // Math operators
    Minus,
    Plus,
    Slash,
    Star,

    // Operators
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Logical operators
    And,
    Or,

    // Literals.
    Identifier,
    Str, // String is a reserved keyword in rust
    Number,

    /* Keywords */
    // Break, // Do we need this? Since no loops? should we even have switch case
    If,
    Else,
    False,
    Function,
    Arrow, // =>

    // For,
    While,
    Print,
    Return,
    True,
    Const,
    Ignore,

    // To remove and introduce Tagged union a.k.a optional T for every T
    // Or maybe uniqueness type
    Null,

    Eof,
}
