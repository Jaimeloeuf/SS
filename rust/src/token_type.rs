//  @todo Used for 'impl' of fmt::Display for TokenType enum
use std::fmt;

// PartialEq is used to do comparison instead of match. Refer to Parser struct methods
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // @todo Remove once scanner complete
pub enum TokenType {
  // Single-character tokens.
  Semicolon,
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
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

  // TO replace these with || and &&
  And,
  Or,

  // Add in binary operators

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

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      // @todo Instead of match, should write its literal value instead
      _ => write!(f, "SS: Either Display trait not implemented yet or invalid"),
    }
  }
}
