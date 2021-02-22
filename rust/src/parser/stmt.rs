use super::expr::Expr;
use crate::token::Token;

// Stmts causes side effects or do something, they usually do not evaluate to a Value enum variant
// Using box to handle this Recursive type with nested Expression variants
// #[derive(Debug, Clone)]
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Const(Token, Expr),
    // Var(Token, Expr),
    Block(Vec<Stmt>),

    // Rlox did If(Expr, Box<Stmt>, Box<Option<Stmt>>), instead
    // But Option on the outer layer was easier to unwrap in the interpreter
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),

    // While(Expr, Box<Stmt>),
    // Func(Token, Vec<Token>, Box<Stmt>),
    // Class(Token, Option<Expr>, Vec<Stmt>),

    // Return stmt is a special stmt variant that will be evaluated to a Value variant
    Return(Token, Box<Expr>),
}
