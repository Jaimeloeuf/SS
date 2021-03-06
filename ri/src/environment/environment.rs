use crate::callables::native;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;

use super::error::EnvironmentError;
use crate::value::value::Value;

#[derive(Debug)]
pub struct Environment {
    // @todo Perhaps use a ref to a String instead of this, to avoid cloning the string
    // @todo Perhaps use a Rc<Value> instead of this, to avoid cloning the Value everytime we read
    // Values field is private as it should only be accessed via the given getters and setters
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

/*
    @todo Test this idea out
    Instead of multi level environment and closure cloning to freeze data method,
    Store all data in a single place for all scopes.
    Then when cloning env, only clone the pointers to the data...
    The environment will be used to enforce scoping rules rather then to do that and store data.
*/
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
        env.define(
            "clock".to_string(),
            Value::Func(Rc::new(native::ClockFunc {})),
        );
        // env.define(
        //     "assert".to_string(),
        //     Value::Func(Rc::new(native::assert::new())),
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

    // @todo
    // Update this whole thing to be mutable, so like we always return a reference to the value object
    // But depending on whether we call get or get_mut() we get different kind of references
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut

    // @todo Temporary method used by interpret to check if Value identifier already exists in current scope
    // @todo Will be removed once this class of errors is handled by the scanner/parser
    // Checks if Value identifier is already used in current scope only.
    pub fn in_current_scope(&self, key: &String) -> bool {
        match self.values.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    // Basic access methods that return values wrapped in option variant
    // pub fn get(&self, key: &String) -> Option<Value> {
    //     // Get value from values hashmap, if not found, recursively get value from the enclosing environment/scope till global environment
    //     // @todo Optimize by iteratively walking up the scope chain instead of recursively
    //     match self.values.get(key) {
    //         // @todo Not sure if this is right, but we return a Clone Value every time so that the original value still stays in the hashmap
    //         // @todo Values should be Rc<Value> instead so we can just Rc::clone() it instead of doing a full clone
    //         Some(value) => Some(value.clone()),
    //         None => match &self.enclosing {
    //             Some(enclosing) => enclosing.borrow().get(key),
    //             None => None,
    //         },
    //     }
    // }

    // Private method used by 'get' to get value from within current environment/scope
    fn get_from_current_env(&self, key: &String) -> Result<Value, EnvironmentError> {
        match self.values.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(EnvironmentError::UndefinedIdentifier(key.clone())),
        }
    }

    pub fn get(&self, key: &String, distance: usize) -> Result<Value, EnvironmentError> {
        if distance == 0 {
            // If identifier is in current scope as assigned by the resolver
            self.get_from_current_env(key)
        } else {
            // Else get environment identifier is defined in to get value
            self.get_scope(distance).borrow().get_from_current_env(key)
        }
    }

    // Private method to get a parent scope using distance value
    // Expect a value 1 or bigger, 0 should be handled by caller
    // @todo Technically dont need to check for None, since if at global scope, then distance will be 0 and this will not get called
    fn get_scope(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environment = Rc::clone(self.enclosing.as_ref().unwrap());
        for _ in 1..distance {
            // Split into 2 lines to satisfy borrow checker rules.
            // Can be sure to unwrap as 1..distance will never exceed global scope
            let parent_env = Rc::clone(environment.borrow().enclosing.as_ref().unwrap());
            environment = parent_env;
        }
        environment
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
            None => Err(EnvironmentError::UndefinedIdentifier(key.clone())),
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
            None => Err(EnvironmentError::UndefinedIdentifier(key.clone())),
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
