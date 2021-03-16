use super::Compiler;

use crate::token::TokenType;

// Precedence enum where all these can be converted to usize
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

// A method on compiler struct...
type ParseFn = fn(&mut Compiler);

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

impl ParseRule {
    const fn new(precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix: None,
            infix: None,
            precedence,
        }
    }
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
static rules_table: [ParseRule; 1] = {
    // Same type as rule table, need to initialize it, so using default empty ParseRule
    let rules_array: [ParseRule; 1] = [ParseRule::new(Precedence::None); 1];

    /*
        Insert rules for each token type 1 by 1
        TokenType enum variants converted to usize first before using it to index the array
    */
    rules_array[TokenType::Semicolon as usize] = ParseRule::new(Precedence::None);

    rules_array
};

// Inline method to act like a macro, for calling rules table with a TokenType variant,
// allow caller to call directly without manually casting TokenType variant to usize
#[inline]
pub fn get_rule(token_type: TokenType) -> ParseRule {
    rules_table[token_type as usize]
}
