// @todo Why cant we just use the Value enum directly? Why hold a literal enum that evalutaes to a value enum later?
// @todo Also right now it needs clone, because Token needs this, and Token have the Clone trait derived.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use ref to make sure the values are only borrowed and not moved
        match self {
            Literal::Number(ref number) => write!(f, "{}", number),
            Literal::String(ref string) => write!(f, "{}", string),
            Literal::Bool(ref boolean) => write!(f, "{}", boolean),
            Literal::Null => write!(f, "Null"),
        }
    }
}
