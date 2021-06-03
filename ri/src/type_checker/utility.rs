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
    // @todo If no where else uses this, can inline this into Expr::Const(..) of resolve_expression
    pub fn get_type(&mut self, token: &Token) -> Type {
        // Use lexeme from token as identifier
        let identifier_string = token.lexeme.as_ref().unwrap();

        // Simple optimization, as identifiers are usually defined in the same scope more often than not
        // Able to unwrap directly as a scope is always expected to exists, including the global top level scope
        if let Some(identifier_type) = self.scopes.last_mut().unwrap().get(identifier_string) {
            identifier_type.clone()
        } else {
            // Convert scopes vector into Iter type and reverse it to traverse up from 1 scope above local scope to top level global scope
            // Skip the first scope, which is the local scope since we already check the local scope in the if statement above.
            for ref scope in self.scopes.iter().rev().skip(1) {
                if scope.contains_key(identifier_string) {
                    return scope.get(identifier_string).unwrap().clone();
                }
            }

            panic!("TypeChecker Internal Error: Identifier type not found in all scopes!")
        }
    }

    // @todo Allow types to be passed in, and change it to be inserting the types 1 by 1
    // @todo Perhaps if that is the case, should change it to inline
    // Method to define identifiers used in the global scope
    pub fn define_globals(&mut self, identifiers: Vec<&str>) {
        for id in identifiers {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(id.to_string(), Type::Null);
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
