use std::env;
use std::fs;
// use std::io;

mod eat;
mod hash;
mod keywords;
mod parser;
mod scanner;
mod token;
mod token_type;

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
    let source = fs::read_to_string(filename).expect("Cannot read file");

    // Debug
    println!("Source content:\n{}", source);

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    let scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("Logging out token vector");
    for token in tokens.iter() {
        println!("{}", token.to_string())
    }
    println!("End of token vector");

    // Might also change to remove the mut and give ownership to parse
    let mut parser = parser::parser::Parser::new(tokens);
    parser.parse();
}
