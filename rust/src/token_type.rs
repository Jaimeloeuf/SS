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
    match self {
      // @todo Instead of match, should write its literal value instead
      TokenType::Semicolon => write!(f, "Semicolon"),
      TokenType::LeftParen => write!(f, "LeftParen"),
      TokenType::RightParen => write!(f, "RightParen"),
      TokenType::LeftBrace => write!(f, "LeftBrace"),
      TokenType::RightBrace => write!(f, "RightBrace"),
      TokenType::Comma => write!(f, "Comma"),
      TokenType::Dot => write!(f, "Dot"),
      TokenType::Minus => write!(f, "Minus"),
      TokenType::Plus => write!(f, "Plus"),
      TokenType::Slash => write!(f, "Slash"),
      TokenType::Star => write!(f, "Star"),
      TokenType::Bang => write!(f, "Bang"),
      TokenType::BangEqual => write!(f, "BangEqual"),
      TokenType::Equal => write!(f, "Equal"),
      TokenType::EqualEqual => write!(f, "EqualEqual"),
      TokenType::Greater => write!(f, "Greater"),
      TokenType::GreaterEqual => write!(f, "GreaterEqual"),
      TokenType::Less => write!(f, "Less"),
      TokenType::LessEqual => write!(f, "LessEqual"),
      TokenType::And => write!(f, "And"),
      TokenType::Or => write!(f, "Or"),
      TokenType::Identifier => write!(f, "Identifier"),
      TokenType::Str => write!(f, "Str"),
      TokenType::Number => write!(f, "Number"),
      TokenType::Break => write!(f, "Break"),
      TokenType::If => write!(f, "If"),
      TokenType::Else => write!(f, "Else"),
      TokenType::False => write!(f, "False"),
      TokenType::Function => write!(f, "Function"),
      TokenType::For => write!(f, "For"),
      TokenType::While => write!(f, "While"),
      TokenType::Print => write!(f, "Print"),
      TokenType::Return => write!(f, "Return"),
      TokenType::True => write!(f, "True"),
      TokenType::Const => write!(f, "Const"),
      TokenType::Null => write!(f, "Null"),
      TokenType::Eof => write!(f, "Eof"),

      #[allow(unreachable_patterns)]
      _ => write!(f, "SS: Either Display trait not implemented yet or invalid"),
    }
  }
}
