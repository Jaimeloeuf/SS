use super::Type;

use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::rc::Rc;

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

    // Method to retrieve type of identifier from type table (self) and its parent type tables if any
    pub fn get_type(&self, key: &String) -> Option<Type> {
        // If type of identifier is found in current scope return it immediately
        if let Some(value_type) = self.types.get(key) {
            return Some(value_type.clone());
        }

        // Since type of identifier not found in current scope, get the enclosing type table as starting point to traverse up
        let mut environment = Rc::clone(self.enclosing.as_ref().unwrap());

        // Loop through all type tables looking for the first to contain a type for the identifier
        loop {
            // Return type of identifier when found
            if let Some(value_type) = environment.borrow().types.get(key) {
                return Some(value_type.clone());
            }

            // Breaks out of loop and function if current type table is for top level scope and no type found for identifier
            // @todo Temporarily return to let caller handle it, might integrate this code into utility function instead
            if environment.borrow().enclosing.is_none() {
                return None;
            }

            // If type not found in current scope and not top level scope, set parent scope to current and continue looking
            // Can be sure to unwrap as the previous code already checks if enclosing is none
            //
            // Code split into 2 lines to satisfy borrow checker, alternative is
            // match environment.borrow().enclosing.as_ref() {
            //     Some(parent_env) => environment = Rc::clone(parent_env),
            //     None => return None,
            // };
            let parent_env = Rc::clone(environment.borrow().enclosing.as_ref().unwrap());
            environment = parent_env;
        }
    }
}
