use super::Compiler;

use crate::compiler::CompileError;
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
    // Safe method with panic guard to wrap around unsafe transmute code to convert usize as discriminant to Precedence variant
    // pub fn from_usize(discriminant: usize) -> Precedence {
    //     // Since Precedence enum uses usize, the discriminant must be between 0 and the number of enum variants
    //     // This acts as a "safe" check before executing unsafe code, to ensure that the unsafe code wont fail
    //     // Dont need to check if it is bigger or equals to 0, since the usize type itself forces it to be 0 or bigger.
    //     if discriminant < NUM_OF_PRECEDENCE_VARIANTS {
    //         unsafe { std::mem::transmute(discriminant) }
    //     } else {
    //         // If the discriminant is out of bounds, panic!
    //         panic!("Out of bounds discriminant used to convert to a Precedence enum variant")
    //     }
    // }
}

/*
    Type definition for methods on compiler struct
    There are 2 types of methods, ones that take a can_assign arguement and others that dont
    To have both without changing the function signature of the methods that do not need that argument
    These methods will be wrapped in a function with a function signature that takes in the extra argument and discard it
*/
pub type ParseFnBase = fn(compiler: &mut Compiler) -> Result<(), CompileError>;
pub type ParseFn = fn(compiler: &mut Compiler, can_assign: bool) -> Result<(), CompileError>;

// Need Copy trait for array initialization process in static RULES_TABLE creation process
// Clone trait is needed to derive the Copy trait
#[derive(Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

// Converts ParseFnBase to ParseFn
macro_rules! wrap_parse_fn {
    ($parse_fn:expr) => {
        |compiler: &mut Compiler, _| $parse_fn(compiler);
    };

    // Alternative wrapper using a inline function instead of a closure
    ($parse_fn:expr => INLINE) => {{
        // Although can_assign will be unused, still have to define it unlike a closure definition
        #[inline]
        fn wrapped(compiler: &mut Compiler, _can_assign: bool) -> Result<(), CompileError> {
            $parse_fn(compiler)
        }

        wrapped
    }};
}

// Macro to generate ParseRule struct instantiations instead of using a const function
// Since function pointers are not allowed in const functions so just using a macro directly instead
macro_rules! new_parse_rule {
    // Internal rule, used by the other matches to expand into ParseRule struct instantiation
    (INTERNAL, $precedence_variant:expr, $prefix:expr, $infix:expr) => {
        ParseRule {
            prefix: $prefix,
            infix: $infix,
            precedence: $precedence_variant,
        }
    };

    // For creating ParseRule that dont have any prefix or infix methods
    // $precedence_variant -> Takes a Precedence enum variant
    ($precedence_variant:expr) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: $precedence_variant,
        }
    };

    // 3 different wrapper macros For creating ParseRule that have prefix, infix or both methods
    // $prefix -> Takes a compiler method to parse/compile the prefix part
    // $infix -> Takes a compiler method to parse/compile the infix part
    // $precedence_variant -> Takes a Precedence enum variant
    (None, $infix:expr, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            None,
            Some(wrap_parse_fn!($infix))
        )
    }};
    ($prefix:expr, None, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            Some(wrap_parse_fn!($prefix)),
            None
        )
    }};
    ($prefix:expr, $infix:expr, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            Some(wrap_parse_fn!($prefix)),
            Some(wrap_parse_fn!($infix))
        )
    }};

    // Rules for parser functions that do not need the function wrapping
    (None, SKIP $infix:expr, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            None,
            Some(wrap_parse_fn!($infix))
        )
    }};
    (SKIP $prefix:expr, None, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            Some(wrap_parse_fn!($prefix)),
            None
        )
    }};
    (SKIP $prefix:expr, SKIP $infix:expr, $precedence_variant:expr) => {{
        new_parse_rule!(
            INTERNAL,
            $precedence_variant,
            Some(wrap_parse_fn!($prefix)),
            Some(wrap_parse_fn!($infix))
        )
    }};
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
    rules_array[TokenType::LeftParen as usize] =
        new_parse_rule!(Compiler::grouping, None, Precedence::None);
    rules_array[TokenType::RightParen as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::LeftBrace as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::RightBrace as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::Comma as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Dot as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Semicolon as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::Minus as usize] =
        new_parse_rule!(Compiler::unary, Compiler::binary, Precedence::Term);
    rules_array[TokenType::Plus as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Term);
    rules_array[TokenType::Slash as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Factor);
    rules_array[TokenType::Star as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Factor);

    rules_array[TokenType::Bang as usize] =
        new_parse_rule!(Compiler::unary, None, Precedence::None);
    rules_array[TokenType::BangEqual as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Equality);
    rules_array[TokenType::Equal as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::EqualEqual as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Equality);
    rules_array[TokenType::Greater as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Comparison);
    rules_array[TokenType::GreaterEqual as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Comparison);
    rules_array[TokenType::Less as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Comparison);
    rules_array[TokenType::LessEqual as usize] =
        new_parse_rule!(None, Compiler::binary, Precedence::Comparison);

    rules_array[TokenType::Const as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Identifier as usize] =
        new_parse_rule!(Compiler::identifier_lookup, None, Precedence::None);
    rules_array[TokenType::Str as usize] =
        new_parse_rule!(Compiler::string, None, Precedence::None);
    rules_array[TokenType::Number as usize] =
        new_parse_rule!(Compiler::number, None, Precedence::None);
    rules_array[TokenType::Null as usize] =
        new_parse_rule!(Compiler::literal, None, Precedence::None);

    rules_array[TokenType::And as usize] = new_parse_rule!(None, Compiler::and, Precedence::And);

    // rules_array[TokenType::For as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::While as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::Function as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Return as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::If as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Else as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::Print as usize] = new_parse_rule!(Precedence::None);

    rules_array[TokenType::True as usize] =
        new_parse_rule!(Compiler::literal, None, Precedence::None);
    rules_array[TokenType::False as usize] =
        new_parse_rule!(Compiler::literal, None, Precedence::None);

    rules_array[TokenType::Error as usize] = new_parse_rule!(Precedence::None);
    rules_array[TokenType::Eof as usize] = new_parse_rule!(Precedence::None);

    // rules_array[TokenType::Class as usize] = new_parse_rule!(Precedence::None);
    // rules_array[TokenType::Super as usize] = new_parse_rule!(Precedence::None);
    // rules_array[TokenType::This as usize] = new_parse_rule!(Precedence::None);

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
