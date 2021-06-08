use super::error::ResolvingError;
use super::resolver::Resolver;

use crate::token::Token;

use std::collections::hash_map::HashMap;

// enum Action {
//     Declare,
//     Define,
//     DeclareAndDefine,
// }

impl Resolver {
    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::<String, bool>::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    // Method to define identifiers used in the global scope
    pub fn define_globals(&mut self, identifiers: Vec<&str>) {
        for id in identifiers {
            self.scopes.last_mut().unwrap().insert(id.to_string(), true);
        }
    }

    // Declare that a identifier was found in the current scope
    pub fn declare(&mut self, token: &Token) -> Result<(), ResolvingError> {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        if scope.contains_key(identifier) {
            // Check if there is only 1 scope, meaning in global scope right now
            // Then check if the identifier is a global identifier introduced by the language runtime
            // If true, means user tried to reuse global identifier, thus show specific error type
            // Else it is a identifier reuse error
            Err(if self.scopes.len() == 1
                && self
                    .globals
                    .iter()
                    .any(|&global_identifier| global_identifier == identifier)
            {
                ResolvingError::IdentifierAlreadyUsedGlobally
            } else {
                ResolvingError::IdentifierAlreadyUsed
            }(token.clone(), identifier.clone()))
        } else {
            // Indicate initializer not resolved
            scope.insert(identifier.clone(), false);
            Ok(())
        }
    }

    // Acknowledge that the identifier completed its initialization phase
    pub fn define(&mut self, token: &Token) {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        // Indicate initializer resolved
        scope.insert(identifier.clone(), true);
    }

    // Declare that a identifier was found in the current scope, and set it to have completed initialization
    pub fn declare_and_define(&mut self, token: &Token) -> Result<(), ResolvingError> {
        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();

        // Get lexeme as identifier from token
        let identifier = token.lexeme.as_ref().unwrap();

        if scope.contains_key(identifier) {
            Err(ResolvingError::IdentifierAlreadyUsed(
                token.clone(),
                identifier.clone(),
            ))
        } else {
            // Indicate initializer resolved
            scope.insert(identifier.clone(), true);
            Ok(())
        }
    }

    // Unified function to
    // self.scope(Action::Declare, token)?;
    // self.scope(Action::Define, token)?;
    // self.scope(Action::DeclareAndDefine, token)?;
    //
    // Instead of having seperate method calls
    // fn scope(&mut self, action: Action, token: &Token) -> Result<(), ResolvingError> {
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
    //                 return Err(ResolvingError::IdentifierAlreadyUsed(
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
