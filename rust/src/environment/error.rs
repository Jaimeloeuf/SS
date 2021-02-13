#[derive(Debug)]
pub enum EnvironmentError {
    Undefined(String),
    UndefinedVariable(String),
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EnvironmentError::Undefined(ref name) => write!(f, "Undefined: {}", name),
            EnvironmentError::UndefinedVariable(ref name) => {
                write!(f, "Undefined variable {}", name)
            }
        }
    }
}

impl std::error::Error for EnvironmentError {
    fn description(&self) -> &str {
        match *self {
            EnvironmentError::UndefinedVariable(_) => "UndefinedVariable",
        }
    }
}
