use super::Compiler;

use crate::token::TokenType;

// @todo Make this u8 instead of usize
// repr(usize) is used temporarily, both to document the memory layout to use and for testing with mem::transmute
// Precedence enum, where they are represented with usize so that they can be converted to it, to do C like enum operations
// Derives both Clone and Copy trait as this enum is used as a field type on ParseRule struct, which implements the 2 traits
#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}


impl Precedence {
    // Option
    pub fn from(us: usize) -> Precedence {
        if us >= 0 && us < NUM_OF_PRECEDENCE_VARIANTS {
            // Some(unsafe { mem::transmute(usize) })
            unsafe { std::mem::transmute(us) }
        } else {
            // None
            panic!("")
        }
    }
}

// A method on compiler struct...
pub type ParseFn = fn(&mut Compiler);

// Need Copy trait for array initialization process in static rules_table creation process
// Clone trait is needed to derive the Copy trait
#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

// Macro to generate ParseRule struct instantiations instead of using a const function
// Since function pointers are not allowed in const functions so just using a macro directly instead
macro_rules! new_parse_rule {
    // $precedence_variant -> Takes a Precedence enum variant
    ($precedence_variant:expr) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: $precedence_variant,
        }
    };
}

// The problem with this approach is that,
// 1. Unlike a static rules table, where we access the ParseRule directly with O(1) speed, this function have a much higher runtime cost
// 2. Unlike accessing a static rules table, everytime a rule is requested, a new ParseRule object is created
// fn get_rule(token_type: TokenType) -> ParseRule {
//     match token_type {
//         TokenType::RightParen => ParseRule::new(Precedence::None),
//         _ => panic!("parse rule missing"),
//     }
// }

// This is the same thing as the one used, only difference is that,
// 1. The alternative is more self documenting, as it explicitly inserts the ParseRule using the token type as index
// 2. This is more dangerous, as a change in TokenType enum's ordering will cause wrong ParseRules to be matched to the TokenTypes
// static rule_table: [ParseRule; 1] = [ParseRule::new(Precedence::None)];

// Static so that only 1 instance of this table in memory
// Create the table internally by inserting rules using TokenType as index 1 by 1
// This has no runtime cost as this is a static value evaluted at compile time
static rules_table: [ParseRule; 40] = {
    // Same type as rule table, need to initialize it, so using default empty ParseRule
    let mut rules_array: [ParseRule; 40] = [new_parse_rule!(Precedence::None); 40];

    /*
        Insert rules for each token type 1 by 1
        TokenType enum variants converted to usize first before using it to index the array
    */
    rules_array[TokenType::Semicolon as usize] = new_parse_rule!(Precedence::None);

    rules_array
};

// Inline method to act like a macro (but with type signatures), for calling rules table with a TokenType variant,
// allow caller to call directly without manually casting TokenType variant to usize
// taking it a TokenType ref to prevent consuming/moving token_type value away from caller
// Since behind a shared ref, token_type needs to be cloned before it can be used
// Return ParseRule ref of static lifetime, since ParseRule is stored in the static rules_table, it will be static
#[inline]
pub fn get_rule(token_type: &TokenType) -> &'static ParseRule {
    &rules_table[token_type.clone() as usize]
}
