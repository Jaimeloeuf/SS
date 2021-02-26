use crate::literal::Literal;
use crate::token::Token;

// @todo Add lifetimes to make Literal a ref instead of owning it, so that we dont have to clone it in parsing_trait
// Right now all the operators are cloned tokens, that are passed in here...
// All expressions can be evaluated to a Value enum variant
// Using box to handle this Recursive type with nested Expression variants
// #[derive(Debug, Clone)]
#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),

    // @todo Use Literal or Value variants directly?
    Unary(Token, Box<Expr>),

    // Expressions that saves other expressions/values into the runtime environment identified by a Const's identifier
    // Evaluates to the value
    Const(Token, Option<usize>),

    // Expressions that assign other expressions/values to a variable
    Assign(Token, Box<Expr>, Option<usize>),

    // Logical And/Or boolean operations
    Logical(Box<Expr>, Token, Box<Expr>),

    // Function calls are also expressions that evaluates to a Value
    Call(Box<Expr>, Vec<Expr>, Token),

    Get(Box<Expr>, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token, Option<usize>),
    Super(Token, Token, Option<usize>),
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
            Expr::Assign(ref token, ref expr, _) => write!(f, "(assign {} {})", token, expr),
            Expr::Logical(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Call(ref callee, ref arguments, _) => {
                write!(f, "(call {} {:?})", callee, arguments)
            }
            Expr::Get(ref expr, ref token) => write!(f, "(get {} {})", token, expr),
            Expr::Set(ref expr, ref token, _) => write!(f, "(set {} {})", token, expr),

            // Expr::This(_, _) => write!(f, "this"),
            // Expr::Super(_, ref method, _) => write!(f, "(super {})", method.lexeme),
            _ => write!(f, "Unimplemented display trait for Expr: {:?}", self),
        }
    }
}
