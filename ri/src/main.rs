use clap::Parser as CLI_Parser;
use std::fs;
use std::time::Instant;

mod callables;
mod cli;
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

use cli::Cli;
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
    // Use Clap lib to parse out CLI arguments
    let args = Cli::parse();

    // @todo Get the full file name instead of the relative path
    println!("Entering file '{}'\n", &args.file_path);

    // Only track execution time for debug builds
    #[cfg(debug_assertions)]
    let start_of_main = Instant::now();

    run_file(&args.file_path);

    // @todo To also ran before running the interpreter
    verbosePrintln!("\nCompleted in: {:?}\n", start_of_main.elapsed());
}

// @todo Should return a Result variant too! Can be a Runtime Variant?
/// Function to compile and run a SimpleScript program file
fn run_file(filename: &String) {
    // This reads the whole file into memory, however large the file may be.
    // Alternative is to use https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html
    let source = fs::read_to_string(filename).expect("RuntimeError - File not found");

    /* Caching mechanism */
    // hash::calculate_hash(&source);

    let tokens = match Scanner::scan_tokens(source) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("-------- Scanning SYNTAX ERROR --------");

            eprintln!("{}\n", e[0]);
            if e.len() > 1 {
                // Because of how it scans tokens, other errors might be falsely detected as the scanner is not synchronized after an error
                eprintln!("---- These might be false positives ----\n");
                for error in e.iter().skip(1) {
                    eprintln!("{}\n", error);
                }
            }

            // Break out of the function
            return;
        }
    };

    // Parse tokens for AST
    let mut ast = match Parser::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("-------- Parsing SYNTAX ERROR --------");

            eprintln!("{}\n", e[0]);
            if e.len() > 1 {
                // Because of how parser scans tokens, other errors might be falsely detected as the error synchronization is not very good
                eprintln!("---- These might be false positives ----\n");
                for error in e.iter().skip(1) {
                    eprintln!("{}\n", error);
                }
            }

            // Break out of the function
            return;
        }
    };

    // Resolve AST and quit on error
    // Mut is used to modify Expr::Const distance value
    if let Err(e) = Resolver::resolve(&mut ast) {
        eprintln!("-------- Resolver ERROR --------");
        eprintln!("{}", e);
        return;
    }

    // Typecheck the AST and quit on error
    if let Err(e) = TypeChecker::check(&mut ast) {
        eprintln!("-------- TypeChecker ERROR --------");
        eprintln!("{}", e);
        return;
    }

    // @todo Interpreter can return a code, which will be used as the program exit code of the interpreter
    // Interpret/Run the AST and quit on error
    if let Some(err) = Interpreter::interpret(ast) {
        eprintln!("-------- Interpreter ERROR --------");
        eprintln!("{}", err);
        return;
    }
}
