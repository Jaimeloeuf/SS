use std::any::Any;

use crate::callables::Callable;
use crate::interpreter::error::RuntimeError;
use crate::interpreter::interpreter::Interpreter;
use crate::value::value::Value;

// cfg_if! {
//     if #[cfg(target_arch = "wasm32")] {
//         mod wasm;
//         use self::wasm::get_current_time;
//     } else {
//         mod default;
//         use self::default::get_current_time;
//     }
// }
mod default;
use self::default::get_current_time;
//

#[derive(Debug)]
pub struct ClockFunc {}

impl Callable for ClockFunc {
    fn to_string(&self) -> String {
        format!("native clock")
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn arity(&self) -> Result<usize, RuntimeError> {
        Ok(0)
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        Ok(Value::Number(get_current_time()))
    }
}
