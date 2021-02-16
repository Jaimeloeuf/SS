use std::collections::hash_map::HashMap;

use super::error::EnvironmentError;
use crate::value::value::Value;

#[derive(Debug)]
pub struct Environment {
    // @todo Perhaps use a ref to a String instead of this, to avoid cloning the string
    // Values field is private as it should only be accessed via the given getters and setters
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::<String, Value>::new(),
            // enclosing: None,
        }
    }

    // Create the global environment / prelude
    pub fn global() -> Environment {
        let mut env = Environment::new();

        // env.define(
        //     "clock".to_string(),
        //     Value::Func(Rc::new(native::ClockFunc::new())),
        // );

        env
    }

    pub fn define(&mut self, key: String, val: Value) {
        self.values.insert(key, val);
    }

    /* ==========================  Start of getter methods  ========================== */
    // Getter methods
    // get methods moves out a clone of the value object, so caller can do whatever it wants with it
    // get_ref methods moves out the immutable reference to the value object in the hashmap, mainly used for accessing constant values

    // Basic access methods that return values wrapped in option variant
    pub fn get(&self, key: &String) -> Option<Value> {
        // @todo Not sure if this is right, but we return a Clone Value every time so that the original value still stays in the hashmap
        Some(self.values.get(key)?.clone())
    }

    // Basic access methods that return values wrapped in option variant
    pub fn get_ref(&self, key: &String) -> Option<&Value> {
        self.values.get(key)
    }

    // 'Safe access method' by wrapping value in a Result variant
    pub fn safe_get(&self, key: &String) -> Result<Value, EnvironmentError> {
        match self.values.get(key) {
            // @todo Not sure if this is right, but we return a Clone Value every time so that the original value still stays in the hashmap
            Some(value) => Ok(value.clone()),
            // None is dealt with here so the caller can chain this
            // Reason is also because the call produces an error type of NoneError if we use the ? operator,
            // so we have to convert it to EnvironmentError to match caller's type signature (RuntimeError)
            None => Err(EnvironmentError::UndefinedVariable(key.clone())),
        }
    }

    // 'Safe access method' by wrapping value in a Result variant
    pub fn safe_get_ref(&self, key: &String) -> Result<&Value, EnvironmentError> {
        match self.values.get(key) {
            // For const values we just return ref since we know that it would not be changed anyways
            Some(value) => Ok(value),
            // None is dealt with here so the caller can chain this
            // Reason is also because the call produces an error type of NoneError if we use the ? operator,
            // so we have to convert it to EnvironmentError to match caller's type signature (RuntimeError)
            None => Err(EnvironmentError::UndefinedVariable(key.clone())),
        }
    }

    /* ==========================  End of getter methods  ========================== */
}
