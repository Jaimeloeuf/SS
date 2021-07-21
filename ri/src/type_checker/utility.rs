use super::structs::Type;
use super::TypeChecker;

use crate::token::Token;

impl TypeChecker {
    // @todo Add lifetime specifier so dont need to clone Type out
    // @todo If no where else uses this, can inline this into Expr::Const(..) of resolve_expression
    pub fn get_type(&mut self, token: &Token) -> Type {
        // Use lexeme from token as identifier
        let identifier_string = token.lexeme.as_ref().unwrap();

        // @todo Look for type in closure first or current env first?
        if let Some(value_type) = self.env.borrow().get_type(identifier_string) {
            return value_type;
        };
        if let Some(ref closure_types) = self.closure_types {
            if let Some(value_type) = closure_types.borrow().get_type(identifier_string) {
                return value_type;
            }
        }
        panic!(
            "Type of '{}' is not found in both current environment and closure",
            identifier_string
        );
    }

    // @todo Allow types to be passed in, and change it to be inserting the types 1 by 1
    // @todo Perhaps if that is the case, should change it to inline
    // Method to define the types of the different identifiers available in the prelude / global scope
    pub fn define_globals(&mut self, identifiers: Vec<&str>) {
        // for id in identifiers {
        //     self.scopes
        //         .last_mut()
        //         .unwrap()
        //         .insert(id.to_string(), Type::Null);
        // }
    }
}
