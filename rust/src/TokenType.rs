// use std::fmt; // @todo Used for 'impl' of fmt::Display for TokenType enum

#[derive(Debug)]
#[allow(dead_code)] // @todo Remove once scanner complete
pub enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or two character tokens.
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals.
  Identifier,
  Str, // String is a reserved keyword in rust
  Number,

  // Keywords.
  And,
  Break,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
}

// impl fmt::Display for TokenType {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     match *self {
//       // Instead of match, should write its literal value instead
//       TokenType::True => write!("true"),
//     }
//   }
// }
