/**
 * Enum for all possible Token Types.
 *
 * Inherited traits:
 * Debug is to make debug printing easier and skip implementing the display trait
 * PartialEq is used to do comparison instead of match. Refer to Parser struct methods
 */
#[derive(Debug, PartialEq)]
pub enum TokenType {
  // @todo
  Error,

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

  // Keywords.
  Break, // Do we need this? Since no loops? should we even have switch case
  // Class,
  // Super,
  // This,
  If,
  Else,
  False,
  Function,
  Arrow, // =>

  For,   // to remove?
  While, // to remove?
  Print, // Shouldnt this be in std:: lib instead?
  Return,
  True,
  Const,

  // What about undefined? Void?
  // Maybe dun allow it, either enforce checking for nulls with the language server,
  // Or force it at the language level by removing Null, and introducing Tagged union a.k.a optional T for every T
  // Else uniqueness type
  Null, // to remove?

  Eof,
}
