use super::Type;

use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;

// Temporary error
pub enum TypeTableError {
    UndefinedIdentifier(String),
}

#[derive(Debug)]
pub struct TypeTable {
    // @todo Perhaps use a ref to a String instead of this, to avoid cloning the string
    // @todo Perhaps use a Rc<Type> instead of this, to avoid cloning the Type everytime we read
    // types field is private as it should only be accessed via the given getters and setters
    types: HashMap<String, Type>,
    enclosing: Option<Rc<RefCell<TypeTable>>>,
}

impl TypeTable {
    // Requires a reference to the enclosing environment, global environment will be the grand parent, and first to enclose on a scope
    pub fn new(enclosing: Option<Rc<RefCell<TypeTable>>>) -> TypeTable {
        TypeTable {
            types: HashMap::<String, Type>::new(),
            enclosing,
        }
    }

    // Create the global environment / prelude
    pub fn global() -> TypeTable {
        // Since global environment is the top level scope, there is no enclosing environment
        let mut type_table = TypeTable::new(None);

        // Rust Reference: https://doc.rust-lang.org/std/prelude/index.html
        // Define types of the prelude (bunch of things auto imported and available at toplevel)
        // env.define(
        //     "clock".to_string(),
        //     Type::Func(Rc::new(native::ClockFunc {})),
        // );
        // env.define(
        //     "assert".to_string(),
        //     Type::Func(Rc::new(native::assert::new())),
        // );

        type_table
    }

    // Can also be called to update value in map
    // Old value will be returned if the value is updated instead of created
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert
    pub fn define(&mut self, key: String, val: Type) {
        println!("inserting '{}' as type {:?}", key, val);
        // Since not supporting variables now, all const declared will be in the current scope
        // So we do not have to traverse up the scope chain to find the environment/scope the variable is created in before assigning
        self.types.insert(key, val);
        print!("after inserting -> ");
        for key in self.types.keys() {
            print!("{}, ", key);
        }
        println!("\n");
    }

    pub fn get_full(&self, key: &String) -> Result<Type, TypeTableError> {
        println!("finding type for: {}", key);

        // If type of identifier is found in current scope
        if let Some(value_type) = self.types.get(key) {
            print!("finding within current scope -> ");
            for key in self.types.keys() {
                print!("{}, ", key);
            }
            println!("\n");
            return Ok(value_type.clone());
        }

        let mut environment = Rc::clone(self.enclosing.as_ref().unwrap());

        // Loop through all scopes looking for the first to contain a type for the identifier
        loop {
            // Split into 2 lines to satisfy borrow checker rules.
            // Can be sure to unwrap as 1..distance will never exceed global scope
            // let parent_env = Rc::clone(environment.borrow().enclosing.as_ref().unwrap());
            // environment = parent_env;

            print!("finding within -> ");
            for key in environment.borrow().types.keys() {
                print!("{}, ", key);
            }
            println!("\n");

            // If type of identifier is found in current scope
            if let Some(value_type) = environment.borrow().types.get(key) {
                return Ok(value_type.clone());
            }

            if environment.borrow().enclosing.is_none() {
                // Temporarily return error to let caller handle it
                return Err(TypeTableError::UndefinedIdentifier(String::from("")));
                // panic!("cannot find the damn type of: {}", key)
            }

            // If value is not found in current scope, set parent scope to current scope and continue looking for it
            let parent_env = Rc::clone(environment.borrow().enclosing.as_ref().unwrap());
            environment = parent_env;
        }

        panic!("Cannot find type of: {}", key)
    }
}
