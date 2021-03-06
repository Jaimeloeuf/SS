use super::structs::Type;
use super::TypeChecker;

use crate::token::Token;

use std::collections::hash_map::HashMap;

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
            panic!(
                "TypeChecker Internal Error: Type of identifier '{}' not found in all scopes!",
                identifier_string
            )
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
}
