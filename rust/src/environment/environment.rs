use std::collections::hash_map::HashMap;

use super::error::EnvironmentError;
use crate::value::value::Value;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
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

    pub fn get(&self, key: &String) -> Result<Value, EnvironmentError> {
        match self.values.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(EnvironmentError::UndefinedVariable(key.clone())),
        }
    }
}
