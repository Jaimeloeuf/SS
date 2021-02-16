use super::expr::Expr;
use crate::token::Token;

// Using box to handle this Recursive type with nested Expression variants
// #[derive(Debug, Clone)]
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Const(Token, Expr),
    // Var(Token, Expr),
    // Block(Vec<Stmt>),
    // If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    // While(Expr, Box<Stmt>),
    // Func(Token, Vec<Token>, Box<Stmt>),
    // Return(Token, Box<Expr>),
    // Class(Token, Option<Expr>, Vec<Stmt>),
}
