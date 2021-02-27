// Enum with all the possible variants of a Value object in SS as a dynamically typed language

use crate::callables::Callable;
use crate::environment::environment::Environment;
use crate::interpreter::error::RuntimeError;
use crate::literal::Literal;
use crate::parser::stmt::Stmt;
use crate::token_type::TokenType;
use crate::Interpreter;

use super::value::Value;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Function {
    declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(statement: Stmt) -> Function {
        Function {
            declaration: statement,
            closure: Rc::new(RefCell::new(Environment::new(None))),
        }
    }
}

impl Callable for Function {
    fn to_string(&self) -> String {
        let name_token = match &self.declaration {
            Stmt::Func(ref name_token, _, _) => name_token,
            unmatched_stmt_variant => {
                // @todo Remove use of debug printing once stmt implements Display trait
                // return Err(RuntimeError::InternalError(format!(
                //     "Function must be Stmt::Func, found: {:?}",
                //     unmatched_stmt_variant,
                // )));
                panic!(format!(
                    "Function must be Stmt::Func, found: {:?}",
                    unmatched_stmt_variant,
                ))
            }
        };

        match name_token.literal.as_ref().unwrap() {
            Literal::String(ref string) if name_token.token_type == TokenType::Identifier => {
                // Maybe instead of 'user' as function type, use 'ss' to indicate function is defined in SS
                format!("<user> {}", string.to_string())
            }
            _ => panic!("Function token missing string identifier...?!?"),
        }
    }

    fn arity(&self) -> usize {
        0
    }

    // @todo Read https://stackoverflow.com/a/33687996/275442
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        mut arguements: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        // Destructure out Stmt::Function items to use
        let (name_token, parameters, body) = match &self.declaration {
            Stmt::Func(ref name_token, ref parameters, ref body) => (name_token, parameters, body),
            unmatched_stmt_variant => {
                // @todo Remove use of debug printing once stmt implements Display trait
                return Err(RuntimeError::InternalError(format!(
                    "Function must be Stmt::Func, found: {:?}",
                    unmatched_stmt_variant,
                )));
            }
        };

        // Get body statement from function body's Stmt::Block
        let body = match &**body {
            Stmt::Block(ref statement) => statement,
            unmatched_stmt_variant => {
                // Might change to support inline/anonymous functions
                // @todo Remove use of debug printing once stmt implements Display trait
                return Err(RuntimeError::InternalError(format!(
                    "Function body must be a Block Statement, found: {:?}",
                    unmatched_stmt_variant
                )));
            }
        };

        // Set current scope (interpreter.env) as the enclosing scope of the new scope
        let mut environment = Environment::new(Some(Rc::clone(&interpreter.env)));

        // @todo Optimize the loop
        // Insert all the arguments into the new environment/scope of the function
        for (index, token) in parameters.iter().enumerate() {
            // Assume literal always exists
            let parameter_name = match token.literal.as_ref().unwrap() {
                Literal::String(ref string) => string,
                _ => {
                    return Err(RuntimeError::InternalError(format!(
                        "Function parameter token missing String literal!"
                    )))
                }
            };

            // Use clone since parameter_name String is still in the Literal and arguement Values are still owned by the Vector
            // environment.define(parameter_name.clone(), arguements[index].clone())

            // If I remove from vec instead of clone, I technically dont even need index anymore
            // And also this will introduce new issues
            // Need to check if there are parameters but no arguments, then skip it and pass in Null?
            environment.define(parameter_name.clone(), arguements.remove(0))
        }

        match interpreter.interpret_block(body, environment)? {
            Some(result) => Ok(result),
            None => Ok(Value::Null),
        }
    }
}
