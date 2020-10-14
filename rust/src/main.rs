use std::env;
use std::fs;

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

    println!("Source content:\n{}", source);
}
