use std;
use std::any::Any;
pub mod native;

use crate::interpreter::error::RuntimeError;
use crate::interpreter::interpreter::Interpreter;
use crate::value::value::Value;

pub trait Callable: std::fmt::Debug {
    // 1 more field for name?

    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        values: Vec<Value>,
    ) -> Result<Value, RuntimeError>;

    // @todo Read https://stackoverflow.com/a/33687996/275442
    fn as_any(&self) -> &Any;
}
