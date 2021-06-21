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
mod type_checker;
mod value;

use interpreter::interpreter::Interpreter;
use parser::parser_struct::Parser;
use resolver::resolver::Resolver;
use scanner::scanner_struct::Scanner;
use type_checker::TypeChecker;

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

    // for arg in &args {
    //     // Use this to look for flags passed in as args
    // }

    let filename = &args[1];
    // @todo Get the full file name instead of the relative path
    println!("Entering file '{}'\n", filename);

    run_file(&filename);

    // @todo To also ran before running the interpreter
    verbosePrintln!("\nCompleted in: {:?}\n", start_of_main.elapsed());
}

// @todo Should return a Result variant too! Can be a Runtime Variant?
fn run_file(filename: &String) {
    let source = fs::read_to_string(filename).expect("RuntimeError - File not found");

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    let tokens = match Scanner::scan_tokens(source) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("------ Scanning SYNTAX ERROR ------");
            for error in e.iter() {
                eprintln!("{}\n", error);
            }
            return;
        }
    };

    // Parse tokens for AST
    let mut ast = match Parser::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("------ Parsing SYNTAX ERROR ------");
            for error in e.iter() {
                eprintln!("{}\n", error);
            }
            return;
        }
    };

    // Resolve AST and quit on error
    // Mut is used to modify Expr::Const distance value
    if let Err(e) = Resolver::resolve(&mut ast) {
        eprintln!("------ Resolver ERROR ------");
        eprintln!("{}", e);
        return;
    }

    // Typecheck the AST and quit on error
    if let Err(e) = TypeChecker::check(&mut ast) {
        eprintln!("------ TypeChecker ERROR ------");
        eprintln!("{}", e);
        return;
    }

    // @todo Interpreter can return a code, which will be used as the program exit code of the interpreter
    // Interpret/Run the AST and quit on error
    if let Some(err) = Interpreter::interpret(ast) {
        eprintln!("------ Interpreter ERROR ------");
        eprintln!("{}", err);
        return;
    }
}
