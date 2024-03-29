use crate::callables::Callable;
use crate::environment::environment::Environment;
use crate::interpreter::error::RuntimeError;
use crate::parser::stmt::Stmt;
use crate::Interpreter;

use super::value::Value;

use std::cell::RefCell;
use std::rc::Rc;

// Function struct will not implement Display trait, instead it will impl the to_string method of the Callable trait
// And Value will implement a Display trait for Value::Func using the to_string method of Callables

#[derive(Debug)]
pub struct Function {
    declaration: Stmt,

    // This is the env surrounding the function definition NOT THE ENV surrounding the function call
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(statement: Stmt, closure: Rc<RefCell<Environment>>) -> Function {
        Function {
            declaration: statement,

            // Right now closure is simply a pointer to the environment when Function created,
            // and identifiers are accessed with scope distance value so we dont need to freeze this environment to prevent modification
            // Alternative is to either freeze current environment at runtime, or create a new environment on every value declaration
            closure,
        }
    }
}

impl Callable for Function {
    fn to_string(&self) -> String {
        match &self.declaration {
            Stmt::Func(ref name_token, _, _) => {
                if let Some(function_identifier) = name_token.lexeme.as_ref() {
                    // Function type is 'ss' to indicate that the function is defined in SS instead of native code.
                    // So both user defined functions and standard library in SS will both be in this category
                    format!("ss: {}", function_identifier.to_string())
                } else {
                    panic!("InternalError: Function token missing string identifier...?!?")
                }
            }
            Stmt::AnonymousFunc(_, _) => format!("ss: [anonymous]"),

            _ => panic!("InternalError: Function cannot be: {}", self.declaration),
        }
    }

    fn arity(&self) -> Result<usize, RuntimeError> {
        match &self.declaration {
            Stmt::Func(_, ref parameters, _) => Ok(parameters.len()),
            Stmt::AnonymousFunc(ref parameters, _) => Ok(parameters.len()),

            _ => panic!("InternalError: Function cannot be: {}", self.declaration),
        }
    }

    // This method handles the creation of a new environment/scope for the function's code to execute in
    fn call(
        &self,
        interpreter: &mut Interpreter,
        mut arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        // Destructure out Stmt::Function items to use
        let (parameters, body) = match &self.declaration {
            Stmt::Func(_, ref parameters, ref body) => (parameters, body),
            Stmt::AnonymousFunc(ref parameters, ref body) => (parameters, body),

            _ => panic!("InternalError: Function cannot be: {}", self.declaration),
        };

        // Get body statement from function body's Stmt::Block
        let body = match &**body {
            Stmt::Block(ref statement, _) => statement,
            unmatched_stmt_variant => {
                // Might change to support inline/anonymous functions
                return Err(RuntimeError::InternalError(format!(
                    "Function body must be a Block Statement, found: {}",
                    unmatched_stmt_variant
                )));
            }
        };

        // Set closure scope as the enclosing scope of the new scope instead of interpreter.env,
        // Because closure scope values are "fixed" on definition and not execution.
        let mut environment = Environment::new(Some(Rc::clone(&self.closure)));

        // @todo Optimize the loop and change .remove(0) to pop()
        // Insert all the arguments into the new environment/scope of the function
        for (index, token) in parameters.iter().enumerate() {
            if let Some(ref parameter_name) = token.lexeme {
                // Use clone since parameter_name String is still in the Literal and argument Values are still owned by the Vector
                // environment.define(parameter_name.clone(), arguments[index].clone())
                //
                // If I remove from vec instead of clone, I technically dont even need index anymore
                // And also this will introduce new issues
                // Need to check if there are parameters but no arguments, then skip it and pass in Null?
                environment.define(parameter_name.clone(), arguments.remove(0))
            } else {
                panic!("Function parameter token missing String literal!");
            }
        }

        // The interpret_block method is shared with Stmt::Block arm of interpret_stmt, and so it has a return Type of Option<Value>
        // like interpret_stmt because not all Stmt evaluates to a Value, therefore, we unwrap it here first and do extra processing
        // before returning, because Callable.call method for using functions as expressions, is expected to always return a value.
        //
        // Internally, interpret_block calls interpret_stmt for every single statement in the block
        // When there is a return statement, the return arm of interpret_stmt returns a value wrapped in the Value::Return variant.
        // This extra code wrapping interpret_block ensures that function.call of Callable.call trait ALWAYS returns a value,
        // And the value returned will never be a Value::Return variant
        Ok(match interpreter.interpret_block(body, environment)? {
            Some(result) => match result {
                Value::Return(value) => *value,
                _ => Value::Null,
            },
            None => Value::Null,
        })

        // Cleaner alternative waiting for the 'if let' gaurd RFC to pass --> https://github.com/rust-lang/rust/issues/51114
        // Ok(match interpreter.interpret_block(body, environment)? {
        //     Some(result) if let Value::Return(value) = result => *value,
        //
        //     // _ instead of None, because we also want to match the case where Some(result) is not of Value::Return type
        //     _ => Value::Null,
        // })
    }
}
