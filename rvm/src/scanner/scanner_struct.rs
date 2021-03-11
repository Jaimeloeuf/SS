// Alternative Scanner struct from Clox using a pointer directly instead of holding the index of the vector
// This is for optimization, as dereferencing a pointer is faster then doing pointer arithmetic with the index before element access
// pub struct Scanner {
//     pub start: *const char,
//     pub current: *const char,
//     pub line: usize,
// }

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}
