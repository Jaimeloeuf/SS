use super::error::TypeCheckerError;
use super::structs::Type;
use super::TypeChecker;

use crate::token::Token;

use std::collections::hash_map::HashMap;

// enum Action {
//     Declare,
//     Define,
//     DeclareAndDefine,
// }

impl TypeChecker {
    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::<String, Type>::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    // @todo Add lifetime specifier so dont need to clone Type out
    pub fn get_type(&mut self, token: &Token) -> Type {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        // Get type from scope hashmap, unwrap directly as resolver has already garunteed that the value exists
        scope.get(identifier).unwrap().clone()
    }

    // Method to define identifiers used in the global scope
    pub fn define_globals(&mut self, identifiers: Vec<&str>) {
        for id in identifiers {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(id.to_string(), Type::Null);
        }
    }

    // Declare that a identifier was found in the current scope
    pub fn declare(&mut self, token: &Token) {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        // Indicate initializer not resolved
        scope.insert(identifier.clone(), Type::Null);
    }

    // Acknowledge that the identifier completed its initialization phase
    pub fn define(&mut self, token: &Token, value_type: Type) {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        // Indicate initializer resolved
        scope.insert(identifier.clone(), value_type);
    }

    // Declare that a identifier was found in the current scope
    pub fn declare_and_define(&mut self, token: &Token) -> Result<(), TypeCheckerError> {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        if scope.contains_key(identifier) {
            Err(TypeCheckerError::IdentifierAlreadyUsed(
                token.clone(),
                identifier.clone(),
            ))
        } else {
            // Indicate initializer resolved
            scope.insert(identifier.clone(), Type::Null);
            Ok(())
        }
    }

    // Unified function to
    // self.scope(Action::Declare, token)?;
    // self.scope(Action::Define, token)?;
    // self.scope(Action::DeclareAndDefine, token)?;
    //
    // Instead of having seperate method calls
    // fn scope(&mut self, action: Action, token: &Token) -> Result<(), TypeCheckerError> {
    //     // A scope is always expected to exists, including the global top level scope
    //     let scope = self.scopes.last_mut().unwrap();

    //     // Get lexeme as identifier from token
    //     let identifier = token.lexeme.as_ref().unwrap();

    //     Ok(match action {
    //         Action::Define => {
    //             // Indicate initializer resolved
    //             scope.insert(identifier.clone(), true);
    //         }
    //         Action::Declare | Action::DeclareAndDefine => {
    //             if scope.contains_key(identifier) {
    //                 return Err(TypeCheckerError::IdentifierAlreadyUsed(
    //                     token.clone(),
    //                     identifier.clone(),
    //                 ));
    //             } else {
    //                 scope.insert(
    //                     identifier.clone(),
    //                     match action {
    //                         Action::Declare => false, // Indicate initializer not resolved
    //                         Action::DeclareAndDefine => true, // Indicate initializer resolved
    //                         Action::Define => panic!("Internal Error Action::Define?!?"),
    //                     },
    //                 );
    //             };
    //         }
    //     })
    // }
}
