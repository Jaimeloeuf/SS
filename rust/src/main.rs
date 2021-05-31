use std::env;
use std::fs;
use std::time::Instant;

mod callables;
mod environment;
mod hash;
mod interpreter;
mod keywords;
mod literal;
mod parser;
mod resolver;
mod scanner;
mod token;
mod token_type;
mod value;

use interpreter::interpreter::Interpreter;
use parser::parser_struct::Parser;
use resolver::resolver::Resolver;
use scanner::scanner_struct::Scanner;

// Macro wrapping around println! macro that only prints in debug builds or if verbose/debugging flag is set
#[macro_export]
macro_rules! verbosePrintln {
    // Accept any number of arguments
    ($($string:expr), *) => {{
        // Only do this for debug builds, might add additonal debug flag to run this in verbose/debugging mode only
        #[cfg(debug_assertions)]
        println!($($string,)*); // Use all arguments and seperate them with a comma
    }};
}

fn main() {
    let start_of_main = Instant::now();

    verbosePrintln!("SS version: 0.0.1");
    let args: Vec<String> = env::args().collect();

    // Use this to get flags as args
    // let arg = &args[1];
    // println!("Searching for {}", arg);

    let filename = &args[1];
    // @todo Get the full file name instead of the relative path
    println!("Entering file '{}'", filename);

    read_file(&filename);

    // @todo To also ran before running the interpreter
    verbosePrintln!("Completed in: {:?}", start_of_main.elapsed());
}

// @todo Should return a Result variant too! Can be a Runtime Variant?
fn read_file(filename: &String) {
    let source = fs::read_to_string(filename).expect("RuntimeError - File not found");

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    // @todo Instead use ? operator, to let it bubble up
    // And these components, scanner, parser, resolver, interpreter, will have their errors converted to a SSError enum
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
        } else if let Ok(mut ast) = abstract_syntax_tree {
            // @todo Should be named statements instead of ast
            verbosePrintln!("AST generated");

            // for stmt in ast.iter() {
            //     println!("{:?}", stmt);
            // }
            // println!();

            // Mut is used to modify Expr::Const distance value
            let resolver = Resolver::resolve(&mut ast);
            if let Err(e) = resolver {
                println!("{}", e);
                panic!("Resolver failed");
            }

            // @todo Return errors if any?
            // @todo Interpreter can return a code, which will be used as the program exit code of the interpreter
            if let Some(err) = Interpreter::interpret(ast) {
                println!("{}", err);
            }
        }
    }
}
