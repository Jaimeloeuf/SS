use crate::literal::Literal;
use crate::token::Token;

// Using box to handle this Recursive type with nested Expression variants
// #[derive(Debug, Clone)]
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Const(Token, Option<usize>),
    Assign(Token, Box<Expr>, Option<usize>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>, Token),
    Get(Box<Expr>, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token, Option<usize>),
    Super(Token, Token, Option<usize>),
}

// Temporary display trait for debugging
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.to_string(), left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Literal(ref literal) => write!(f, "{}", literal),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator.to_string(), expr),
            Expr::Const(ref token, _) => write!(f, "(Const {})", token.to_string()),
            Expr::Assign(ref token, ref expr, _) => {
                write!(f, "(assign {} {})", token.to_string(), expr)
            }
            Expr::Logical(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.to_string(), left, right)
            }
            Expr::Call(ref callee, ref arguments, _) => {
                write!(f, "(call {} {:?})", callee, arguments)
            }
            Expr::Get(ref expr, ref token) => write!(f, "(get {} {})", token.to_string(), expr),
            Expr::Set(ref expr, ref token, _) => write!(f, "(set {} {})", token.to_string(), expr),
            // Expr::This(_, _) => write!(f, "this"),
            // Expr::Super(_, ref method, _) => write!(f, "(super {})", method.lexeme),
            _ => write!(f, "Unimplemented display trait for Expr: {:?}", self),
        }
    }
}
