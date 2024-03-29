use crate::literal::Literal;
use crate::token::Token;

use super::stmt::Stmt;

// @todo Add lifetimes to make Literal a ref instead of owning it, so that we dont have to clone it in parsing_trait
// Right now all the operators are cloned tokens, that are passed in here...
// All expressions can be evaluated to a Value enum variant
// Using box to handle this Recursive type with nested Expression variants
// Inherit Clone trait because Stmt enum contains this enum variants, and Stmt enum needs to inherit Clone trait for now.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),

    // @todo Use Literal or Value variants directly?
    Unary(Token, Box<Expr>),

    // Expressions that saves other expressions/values into the runtime environment identified by a Const's identifier
    // Evaluates to the value
    // Const(Token, Option<usize>),
    // @todo Rename to create_const as this name is misleading
    Const(Token, usize),

    // @todo Why cant the logic be here instead of inside Stmt variant? Then the stmt variant will just be like a "expression stmt"
    // Expression to wrap around a Stmt::AnonymousFunc variant as anonymous functions are expressions
    AnonymousFunc(Box<Stmt>),

    // @todo Do we really need the token?
    Array(Token, Vec<Expr>),
    // First element is a Expr::Const identifier that points to the array, the second is an expression that evaluates to the array index
    ArrayAccess(Box<Expr>, Box<Expr>),

    // Expressions that assign other expressions/values to a variable
    // Assign(Token, Box<Expr>, Option<usize>),

    // Logical And/Or boolean operations
    Logical(Box<Expr>, Token, Box<Expr>),

    // Function calls are also expressions that evaluates to a Value
    Call(Box<Expr>, Vec<Expr>, Token),
    //
    // Get(Box<Expr>, Token),
    // Set(Box<Expr>, Token, Box<Expr>),
    // This(Token, Option<usize>),
    // Super(Token, Token, Option<usize>),
}

// Temporary display trait for debugging
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Literal(ref literal) => write!(f, "{}", literal),
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator, expr),
            Expr::Const(ref token, _) => write!(f, "(Const {})", token),
            Expr::AnonymousFunc(ref stmt) => write!(f, "(AnonymousFunction {})", stmt),
            Expr::Array(_, ref elements) => {
                write!(f, "(arr {:?})", elements) // Perhaps use something better then debug print
            }
            Expr::ArrayAccess(ref array, ref index) => {
                write!(f, "(arr-element {} {})", array, index)
            }
            // Expr::Assign(ref token, ref expr, _) => write!(f, "(assign {} {})", token, expr),
            Expr::Logical(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Call(ref callee, ref arguments, _) => {
                write!(f, "(call {} {:?})", callee, arguments)
            }

            // Expr::Get(ref expr, ref token) => write!(f, "(get {} {})", token, expr),
            // Expr::Set(ref expr, ref token, _) => write!(f, "(set {} {})", token, expr),
            // Expr::This(_, _) => write!(f, "this"),
            // Expr::Super(_, ref method, _) => write!(f, "(super {})", method.lexeme),
        }
    }
}

// Alternative with Lifetime assigned to the values
// #[derive(Debug)]
// pub enum Expr<'a> {
//     Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
//     Grouping(Box<Expr<'a>>),
//     Literal(Literal),
//     Unary(&'a Token, Box<Expr<'a>>),
//     Const(&'a Token, Option<usize>),
//     Assign(&'a Token, Box<Expr<'a>>, Option<usize>),
//     Logical(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
//     Call(Box<Expr<'a>>, Vec<Expr<'a>>, &'a Token),
// }
