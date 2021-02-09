use std::env;
use std::fs;
use std::time::Instant;

mod hash;
mod interpreter;
mod keywords;
mod literal;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;

use interpreter::interpreter::Interpreter;
use parser::parser_struct::Parser;
use scanner::scanner_struct::Scanner;

fn main() {
    let start_of_main = Instant::now();

    let args: Vec<String> = env::args().collect();

    // Use this to get flags as args
    // let arg = &args[1];
    // println!("Searching for {}", arg);

    let filename = &args[1];
    println!("Entering file '{}'", filename);

    read_file(&filename);

    println!("Completed in: {:?}", start_of_main.elapsed());
}

fn read_file(filename: &String) {
    let source = fs::read_to_string(filename).expect("Cannot read file");

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    let tokens = Scanner::scan_tokens(source);

    if let Err(e) = tokens {
        println!("Program stopped due to Scanning SYNTAX ERROR.");

        for error in e.iter() {
            println!("{}\n", error);
        }

        return;
    } else if let Ok(tokens) = tokens {
        // println!("Logging out token vector");
        // for token in tokens.iter() {
        //     println!("{}", token.to_debug_string());
        // }
        // println!("End of token vector");

        // @todo Should not nest call to parser and interpreter like this

        // Give tokens to parse directly
        let abstract_syntax_tree = Parser::parse(tokens);

        if let Err(e) = abstract_syntax_tree {
            println!("Program stopped due to SYNTAX ERROR.");

            for error in e.iter() {
                println!("{}\n", error);
            }
        } else if let Ok(ast) = abstract_syntax_tree {
            println!("AST generated");

            for stmt in ast.iter() {
                println!("{:?}", stmt);
            }
            println!();

            // Return errors if any?
            // Should be name statements instead of ast
            Interpreter::interpret(ast);
        }
    }
