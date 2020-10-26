use std::env;
use std::fs;
// use std::io;

mod Token;
mod TokenType;
mod eat;
mod hash;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use this to get flags as args
    // let arg = &args[1];
    // println!("Searching for {}", arg);

    let filename = &args[1];
    println!("Entering file '{}'", filename);

    read_file(&filename);
}

fn read_file(filename: &String) {
    let mut source = fs::read_to_string(filename).expect("Cannot read file");

    // Debug
    println!("Source content:\n{}", source);

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    let mut scanner = scanner::Scanner::new(&mut source);
    scanner.scan_tokens();
}
