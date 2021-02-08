use std;

pub struct ScannerError {
    pub line: usize,
    pub description: String,
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[line {}] ScannerError: {}", self.line, self.description)
    }
}
