#[derive(Debug)]
pub struct ScannerError {
    pub line: usize,
    pub description: String,
}
