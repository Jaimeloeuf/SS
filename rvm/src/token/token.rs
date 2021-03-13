use super::TokenType;

// Alternative way from Clox using a pointer directly instead of holding the index of the vector
// This is for optimization, as dereferencing a pointer is faster then doing pointer arithmetic with the index before element access
// #[derive(Debug)]
// pub struct Token {
//   pub token_type: TokenType,
//   pub start: *const char,
//   pub length: usize,
//   pub line: usize,
// }

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub start: usize,
  pub length: usize,
  pub line: usize,
}
