#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Literal {
    pub fn to_string(&self) -> String {
        // Not sure if this will work for all variants..?
        self.to_string()
        // match self {
        //     Number => self.to_string(),
        //     String => self.to_string(),
        //     Bool => self.to_string(),
        //     Null => "Null".to_string(),
        // }
    }
}
