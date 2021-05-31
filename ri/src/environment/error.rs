pub enum EnvironmentError {
    UndefinedIdentifier(String),
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EnvironmentError::UndefinedIdentifier(ref name) => {
                write!(f, "Undefined variable {}", name)
            }
        }
    }
}
