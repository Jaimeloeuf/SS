use std::any::Any;

use crate::interpreter::error::RuntimeError;
use crate::interpreter::interpreter::Interpreter;
use crate::value::value::Value;

// Using trait as a common denominator to be implemented by both native functions and user defined functions
// So that we can deal with functions using the same interface regardless of whether they are native or user defined functions.
pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;

    // Call method will be called by Expr::Call arm of interpret_expr method of interpreter
    // Since function calls are expressions, they are expected to ALWAYS evaluate to a Value
    // Thus there is no Option wrapping for Value and this method shares the return Type signature of interpret_expr
    fn call(
        &self,
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<Value, RuntimeError>;

    // @todo Read https://stackoverflow.com/a/33687996/275442
    fn as_any(&self) -> &Any;

    // Panic for now since technically this is an internal error with the implementation
    fn to_string(&self) -> String {
        // format!("<none> Anonymous")
        // String::from("UNIMPLEMENTED .to_string() method")
        panic!("UNIMPLEMENTED .to_string() method")
    }
}
