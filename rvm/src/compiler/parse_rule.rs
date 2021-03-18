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

// Use last enum variant for its discriminant to calculate the number of variants in the enum
const NUM_OF_PRECEDENCE_VARIANTS: usize = (Precedence::Primary as usize) + 1;

// This array is used to convert usize into a Precedence enum variant using the usize as discriminant of the enum
// This has no runtime cost as this is a static value evaluted at compile time compared to a match statement from usize to variant
// Access is constant time too since this will be a direct array indexing operation
pub static USIZE_TO_PRECEDENCE: [Precedence; NUM_OF_PRECEDENCE_VARIANTS] = {
    // Same type as rule table, need to initialize it, so using default empty ParseRule
    let mut precedence_array = [Precedence::None; NUM_OF_PRECEDENCE_VARIANTS];

    /*
        Explicitly insert precedence for each token type 1 by 1 using the precedence variant itself as index
        This prevents errors caused with directly instantiating the array in the correct order
        Precedence enum variants are converted to usize first before using it to index the array
    */
    precedence_array[Precedence::None as usize] = Precedence::None;
    precedence_array[Precedence::Assignment as usize] = Precedence::Assignment;
    precedence_array[Precedence::Or as usize] = Precedence::Or;
    precedence_array[Precedence::And as usize] = Precedence::And;
    precedence_array[Precedence::Equality as usize] = Precedence::Equality;
    precedence_array[Precedence::Comparison as usize] = Precedence::Comparison;
    precedence_array[Precedence::Term as usize] = Precedence::Term;
    precedence_array[Precedence::Factor as usize] = Precedence::Factor;
    precedence_array[Precedence::Unary as usize] = Precedence::Unary;
    precedence_array[Precedence::Call as usize] = Precedence::Call;
    precedence_array[Precedence::Primary as usize] = Precedence::Primary;

    precedence_array
};

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

// Need Copy trait for array initialization process in static RULES_TABLE creation process
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
    // For creating ParseRule that dont have any prefix or infix methods
    // $precedence_variant -> Takes a Precedence enum variant
    ($precedence_variant:expr) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: $precedence_variant,
        }
    };

    // For creating ParseRule that have prefix, infix or both methods
    // $prefix -> Takes a compiler method to parse/compile the prefix part
    // $infix -> Takes a compiler method to parse/compile the infix part
    // $precedence_variant -> Takes a Precedence enum variant
    ($prefix:expr, $infix:expr, $precedence_variant:expr) => {
        ParseRule {
            prefix: $prefix,
            infix: $infix,
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

// Use last enum variant's discriminant to calculate number of variants in the enum
// Used during RULES_TABLE initialization as the number of elements
const NUM_OF_TOKENTYPE_VARIANTS: usize = (TokenType::Eof as usize) + 1;

// This array is used to map a TokenType to a ParseRule enum variant without using a hashmap
// Static so that only 1 instance of this table in memory
// Create the table internally by inserting rules using TokenType as index 1 by 1
// This has no runtime cost as this is a static value evaluted at compile time
static RULES_TABLE: [ParseRule; NUM_OF_TOKENTYPE_VARIANTS] = {
    // Same type as rule table, need to initialize it, so using default empty ParseRule
    let mut rules_array: [ParseRule; NUM_OF_TOKENTYPE_VARIANTS] =
        [new_parse_rule!(Precedence::None); NUM_OF_TOKENTYPE_VARIANTS];

    /*
        Explicitly insert rules for each token type 1 by 1 using TokenType as index
        This prevents errors caused with directly instantiating the array in the correct order
        TokenType enum variants are converted to usize first before using it to index the array
    */
    rules_array[TokenType::Semicolon as usize] = new_parse_rule!(Precedence::None);

    rules_array
};

// Inline method to act like a macro (but with type signatures), for calling rules table with a TokenType variant,
// allow caller to call directly without manually casting TokenType variant to usize
// taking it a TokenType ref to prevent consuming/moving token_type value away from caller
// Since behind a shared ref, token_type needs to be cloned before it can be used
// Return ParseRule ref of static lifetime, since ParseRule is stored in the static RULES_TABLE, it will be static
#[inline]
pub fn get_rule(token_type: &TokenType) -> &'static ParseRule {
    &RULES_TABLE[token_type.clone() as usize]
}