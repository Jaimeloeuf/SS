use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;

use super::error::EnvironmentError;
use crate::value::value::Value;

#[derive(Debug)]
pub struct Environment {
    // @todo Perhaps use a ref to a String instead of this, to avoid cloning the string
    // @todo Values field is private as it should only be accessed via the given getters and setters
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // Requires a reference to the enclosing environment, global environment will be the grand parent, and first to enclose on a scope
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Environment {
        Environment {
            values: HashMap::<String, Value>::new(),
            enclosing,
        }
    }

    // Create the global environment / prelude
    pub fn global() -> Environment {
        // Since global environment is the top level scope, there is no enclosing environment
        let mut env = Environment::new(None);

        // Rust Reference: https://doc.rust-lang.org/std/prelude/index.html
        // Define prelude (bunch of things auto imported and available at toplevel)
        // env.define(
        //     "clock".to_string(),
        //     Value::Func(Rc::new(native::ClockFunc::new())),
        // );

        env
    }

    // Can also be called to update value in map
    // Old value will be returned if the value is updated instead of created
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert
    pub fn define(&mut self, key: String, val: Value) {
        // Since not supporting variables now, all const declared will be in the current scope
        // So we do not have to traverse up the scope chain to find the environment/scope the variable is created in before assigning
        self.values.insert(key, val);
    }

    /* ==========================  Start of getter methods  ========================== */
    // Getter methods
    // get methods moves out a clone of the value object, so caller can do whatever it wants with it
    // get_ref methods moves out the immutable reference to the value object in the hashmap, mainly used for accessing constant values

    // Basic access methods that return values wrapped in option variant
    pub fn get(&self, key: &String) -> Option<Value> {
        // Get value from values hashmap, if not found, recursively get value from the enclosing environment/scope till global environment
        // @todo Optimize by iteratively walking up the scope chain instead of recursively
        match self.values.get(key) {
            // @todo Not sure if this is right, but we return a Clone Value every time so that the original value still stays in the hashmap
            Some(value) => Some(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(key),
                None => None,
            },
        }
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

    // Clear map and free as much memory as possible
    pub fn clear(&mut self) {
        // Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
        self.values.clear();
        // Shrinks capacity of map as much as possible while maintaining internal rules (possibly leaves some space for the resize policy)
        self.values.shrink_to_fit();
    }
}
