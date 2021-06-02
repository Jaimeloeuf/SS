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

    /// A stmt that is just an expression with a semicolon.
    /// Usually evaluated for its side effects, e.g. a function call
    Expr(Expr),

    /// Const(identifier_token, initial_value)
    Const(Token, Expr),

    /// A block stmt is just a vector of all the stmts defined in that block
    Block(Vec<Stmt>),

    /// If(condition, stmt_to_run_if_condition_is_true, optional_stmt_to_run_if_condition_is_false)
    ///
    /// Note that the stmts are not necessarily block stmts, as they can be single line stmts without brackets
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),

    /// While(condition, loop_body_stmt)
    ///
    /// Note that loop_body_stmt is not necessarily a block stmt, it can be a single line loop
    While(Expr, Box<Stmt>),

    /// Func(name_token, parameter_tokens, body_as_a_block_stmt)
    Func(Token, Vec<Token>, Box<Stmt>),
    /// The only difference between Func and AnonymousFunc is that AnonymousFunc dont have the name token
    ///
    /// AnonymousFunc will be wrapped in the Expr::AnonymousFunc variant since it is treated as an expression
    AnonymousFunc(Vec<Token>, Box<Stmt>),

    /// Return(keyword_token, return_expression)
    ///
    /// Return stmt is a special stmt variant that will be evaluated to a Value variant,
    /// where the value is the evaluated expr, 'return_expression'
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
            Stmt::AnonymousFunc(ref parameters, ref body) => {
                write!(f, "(funcall [anonymous] {:?} {})", parameters, body)
            }
            Stmt::Return(_, ref expr) => {
                write!(f, "(return {})", expr)
            }

            _ => write!(f, "Unimplemented display trait for Stmt::{:?}", self),
        }
    }
}
