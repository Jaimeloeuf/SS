use super::expr::Expr;
use crate::token::Token;

// Stmts causes side effects or do something, they usually do not evaluate to a Value enum variant
// Some stmts like Return and Block can evaluate to a Value enum variant
//
// Using box to handle this Recursive type with nested Expression variants
// Inherits Clone trait for now because we want multiple code to own Stmt, so the easiest way right now is to clone it ...
// Might be able to do away with Clone trait if all use of Stmt is wrapped in Rc in the future
#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Const(Token, Expr),
    // Var(Token, Expr),
    Block(Vec<Stmt>),

    // Rlox did If(Expr, Box<Stmt>, Box<Option<Stmt>>), instead
    // But Option on the outer layer was easier to unwrap in the interpreter
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),

    While(Expr, Box<Stmt>),

    Func(Token, Vec<Token>, Box<Stmt>),
    // Class(Token, Option<Expr>, Vec<Stmt>),

    // Return stmt is a special stmt variant that will be evaluated to a Value variant
    Return(Token, Box<Expr>),
}

// Temporary display trait for debugging
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::Expr(ref expr) => write!(f, "{}", expr),
            Stmt::Print(ref expr) => write!(f, "(print {})", expr),
            Stmt::Const(ref token, ref expr) => {
                write!(f, "(const {} {})", token.literal.as_ref().unwrap(), expr)
            }
            Stmt::Block(ref statments) => write!(f, "(do {:?})", statments),
            Stmt::If(ref expr, ref if_branch, ref else_branch) => {
                write!(f, "(if {} {} {:?})", expr, if_branch, else_branch)
            }
            Stmt::While(ref expr, ref stmt) => write!(f, "(loop {} {})", expr, stmt),
            Stmt::Func(ref token, ref parameters, ref body) => {
                write!(
                    f,
                    "(funcall {} {:?} {})",
                    token.literal.as_ref().unwrap(),
                    parameters,
                    body
                )
            }
            Stmt::Return(_, ref expr) => {
                write!(f, "(return {})", expr)
            }

            _ => write!(f, "Unimplemented display trait for Stmt::{:?}", self),
        }
    }
}
