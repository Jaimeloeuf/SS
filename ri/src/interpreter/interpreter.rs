use super::error::RuntimeError;
use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::environment::Environment;
use crate::literal::Literal;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token_type::TokenType;
use crate::value::function::Function;
use crate::value::value::Value;

pub struct Interpreter {
    // Env tracks the current environment, changing as the interpreter enter and exit local scopes
    env: Rc<RefCell<Environment>>,
}

// Macro to perform a binary arithmetic operation (+, -, *, /) on 2 operands
macro_rules! arithmetic_binary_op {
    // $operator    -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    // $left_value  -> Left operand
    // $right_value -> Right operand
    // $op_name     -> String literal name for the actual binary operation, used in error output for debugging
    ($operator:tt, $left_value:expr, $right_value:expr, $op_name:literal) => {
        match ($left_value, $right_value) {
            (Value::Number(left_number), Value::Number(right_number)) => Ok(Value::Number(left_number $operator right_number)),
            _ => Err(RuntimeError::TypeError($op_name.to_string())),
        }
    };
}

// Macro to perform a numeric comparison operation (>, >=, <, <=) on 2 operands
macro_rules! numeric_comparison_op {
    // $operator    -> accepts a TokenTree -> Single Token -> Punctuation -> https://doc.rust-lang.org/reference/tokens.html#punctuation
    // $left_value  -> Left operand
    // $right_value -> Right operand
    // $op_name     -> String literal name for the actual binary operation, used in error output for debugging
    ($operator:tt, $left_value:expr, $right_value:expr, $op_name:literal) => {
        match ($left_value, $right_value) {
            (Value::Number(left_number), Value::Number(right_number)) => Ok(Value::Bool(left_number $operator right_number)),
            _ => Err(RuntimeError::TypeError($op_name.to_string())),
        }
    };
}

impl Interpreter {
    // pub fn interpret( stmts: Vec<Stmt>, writer: Rc<RefCell<mut io::Write>>) -> Option<RuntimeError> {
    pub fn interpret(stmts: Vec<Stmt>) -> Option<RuntimeError> {
        let mut interpreter = Interpreter {
            // Why did rlox clone the globals here?
            // The starting environment will always be the global scope
            env: Rc::new(RefCell::new(Environment::global())),
        };

        // Loop through all Expr/Stmt to evaluate and run them, returning any errors
        for stmt in stmts.iter() {
            // Interpreter to stop if there is any runtime error
            if let Err(err) = interpreter.interpret_stmt(stmt) {
                return Some(err);
            }

            // The returned value is only meaningful when using as a repl
            // match interpreter.interpret_stmt(stmt) {
            //     Ok(value) => println!("> {:?}", value),
            //     // Interpreter stop interpreting code once there is any runtime error
            //     Err(err) => return Some(err),
            // }
        }

        None
    }

    // Utility method
    // Used by both Stmt::Block and Value::Function
    // Caller to MOVE in a new environment
    // @todo Remove use of self, then change to interpreter or something, then move this into its own module/file?
    pub fn interpret_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<Option<Value>, RuntimeError> {
        // Get a new Rc<Environment> pointing to the same Environment memory allocation
        // Essentially, get a reference to self.env by cloning a pointer to it and not actually clone the underlying data
        let parent_env = Rc::clone(&self.env);

        // Set the new environment directly onto the struct, so other methods can access it directly
        // @todo Can be better written, by changing all the methods to take current scope as function argument,
        // @todo instead of saving current environment temporarily and attaching the new environment to self.
        self.env = Rc::new(RefCell::new(environment));

        // Execute the statements in the block 1 by 1 until either all executed or an return stmt is executed
        let mut return_value: Option<Value> = None;
        for ref stmt in statements {
            /*
                Right now interpret_stmt returns a Value for these statement variants:
                - return --> Returns value of the evaluated expression on the right
                - expr --> stmt::expr calls interpret_expr which always return a Value, meaning ANY expr::{expr_type} causes a return value
                - block
                - if --> since this is basically conditional block statement
            */
            // @todo Deal with the errors better to drop the memory of env. Will this be done automatically?
            return_value = self.interpret_stmt(stmt)?;

            // If the current statement evaluated to a Value
            if return_value.is_some() {
                // If the current statement is a return statement, its value will be a Value::Return variant
                // The Value::Return variant tells us a return statement was executed, and also holds the return value
                if let Stmt::Return(ref _token, ref _expr) = stmt {
                    // Break out of this loop of statements, in this Block statement
                    // To stop executing code in this block statement
                    //
                    // When this ends, the parent block statement will know what to do, by looking at the return_value and its type
                    // If the return value is of Value::Return, then it will also stop its executing its current loop
                    break;
                }

                // If interpret_stmt returned a Value::Return variant, it means that
                // The last statement must had a nested return, it can't be a return statement since the previous 'if' already checks for it.
                // So for nested returns, stop executing stmts, delete current scope and continue bubbling return value up
                // Value::Return variant will be bubbled up all the way to Function's call method, where it first called interpret_block
                // Inside the call method, it will unwrap actual return value out from Value::Return and pass it back to interpret_expr!
                //
                // Using _ to just match this pattern without using/moving out the nested value
                if let Some(Value::Return(_)) = return_value {
                    break;
                }

                // return_value = None;
                // If it is not a return value set return value to None if we want it to be cleaner.
                // Because whatever is set in 'return_value' will be returned, and will be effectively treated as the return value of a block statement
                //
                // BUT, since we do not allow any return statement outside of function body, and error out in the resolver if there it,
                // Technically situations like this will not arise, so it is fine to skip resetting the value to None.
                // Optimizing it by skipping the assignment for clearing it as we simply wont use any value returned.
            }

            // If we allowed implicit returns...
            // We shouldnt make last statement be the implicit return,
            // Only if last statement is an expression can it be an implicit return...
            //
            // Should probably only allow this in single line anonymous functions ->  [1, 2, 3].map(x => x * 2);
            // But can we possibly just make that syntatic sugar and desugar it to be map(function(x) { return x * 2; })
            // In this case, if we desugar it, we still dont actually need implicit returns
            //
            // if "stmt is last stmt of block" && return_value.is_some() {
            //     break;
            // }
        }

        // Reset parent environment back onto the struct once block completes execution
        // The newly created current environment for this block will be dropped once function exits
        self.env = parent_env;

        // Return the return value of the block if there is any
        Ok(return_value)
    }

    // Expects ref to a Stmt rather than a Stmt, as caller might need to keep ownership of the Stmt value sometimes,
    // even after calling interpret_stmt, e.g. to call interpret_stmt multiple times with the same Stmt.
    // Examples include the body of a loop and body of a function.
    //
    // Returns a Value Option because not every statement evaluates to a Value
    fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, RuntimeError> {
        // @todo Change match to use match *stmt instead of stmt
        // @todo Or change to Rc wraps instead of cloning, to minimize memory used and data duplication
        //
        // Wrap match expression in Ok variant instead of wrapping Value options with Ok variant in every arm
        // Err option inside match expression cannot evaluate and return implicitly due to the Ok wrapping,
        // thus it needs to be explicitly returned to break out of this Ok variant wrapping.
        Ok(match stmt {
            // If a plain expr is being interpreted, should we skip it if not in REPL?
            // It is only useful, if the expression contains a function call or something, that contains side effects,
            // But for other expressions, they can technically be skipped...
            // @todo Perhaps simplify this by wrapping the whole interpret_expr in a Option<Value> too to dont have to unwrap and rewrap here
            Stmt::Expr(ref expr) => Some(self.interpret_expr(expr)?),

            // Block statement, groups statements together in the same scope for execution
            Stmt::Block(ref statements, _) => {
                // Create new environment/scope for current block with existing environment/scope as the parent/enclosing environment
                let current_env = Environment::new(Some(Rc::clone(&self.env)));

                // Since interpret_block shares the same return Type signature as interpret_stmt
                // Return directly to break out of this 'match expression' in Ok variant
                return self.interpret_block(statements, current_env);
            }

            // Function definition statements
            // Create a new Value of Function type and insert into environment
            Stmt::Func(ref name_token, _, _) => {
                // Although the token definitely have a string lexeme if scanned correctly,
                // Rust treats this as a pattern matching context with a refutable pattern, so else case must be handled,
                // Which only happens if scanner failed to save String lexeme for Identifier type Token
                // Reference: https://stackoverflow.com/questions/41573764
                // Since this is a token of Identifier type, lexeme can be used directly
                if let Some(ref function_name) = name_token.lexeme {
                    //
                    // Pass in current environment/scope as the function's closure
                    // closure is defined during function definition
                    // Current environment is passed to Function's constructor as its closure (closures created at function definition)
                    //
                    // Closure environment is defined now, before function itself is saved into the environment,
                    // But the function can still call itself recursively because the function holds a reference to the environment,
                    // Which in the next line is modified to save the function itself.
                    //
                    // At the same time this does not cause certain reference issues/bugs where a local identifier mess up access
                    // Because the resolver pass already assigned a scope distance to every single identifier,
                    // So even if current environment is modified, the same identifier will still be used, using scope distance value stored in the AST
                    // Reference: https://craftinginterpreters.com/resolving-and-binding.html#static-scope
                    let func =
                        Value::Func(Rc::new(Function::new(stmt.clone(), Rc::clone(&self.env))));

                    self.env.borrow_mut().define(function_name.clone(), func);
                    None
                } else {
                    return Err(RuntimeError::InternalError(format!(
                        "Parsing error: Function token missing string literal"
                    )));
                }
            }

            // Return statement calls intepret_expr to interpret the expression on its right.
            // Wrap the value of the evaluated expression, inside the special Value::Return variant
            // So as to differentiate this from a normal expression statement value.
            // So the interpreter can stop further execution and just return it to the interpreter method that called it
            // 'return;' will be auto parsed to 'return Value::Null;' by parser
            Stmt::Return(_, ref expr) => Some(Value::Return(Box::new(self.interpret_expr(expr)?))),

            Stmt::If(ref condition, ref true_branch, ref else_branch) => {
                let branch = if self.interpret_expr(condition)?.bool_or_err(
                    "Invalid condition value type, only Boolean values can be used as conditionals!"
                )? {
                    true_branch
                } else {
                    // Only return else_branch if any, else end function
                    match else_branch {
                        Some(ref else_branch) => else_branch,
                        _ => return Ok(None), // Return to break out of this expr passed into interpret_stmt method call
                    }
                };
                self.interpret_stmt(branch)?
                // Using pattern matching instead of checking with built in .bool_or_err() method,
                // to use a more specific RuntimeError for invalid condition types instead of TypeError
                // self.interpret_stmt(match self.interpret_expr(condition)? {
                //     Value::Bool(true) => true_branch,
                //     // Only return else_branch if any, else end function
                //     Value::Bool(false) => match else_branch {
                //         Some(ref else_branch) => else_branch,
                //         _ => return Ok(None), // Return to break out of this expr passed into interpret_stmt method call
                //     },
                //     // Throws error if condition does not evaluates to a Value of Bool type
                //     // This is because SS will not support truthy and falesy values, so none Bool values cannot cast to Bool trues and falses
                //     invalid_condition_type => return Err(RuntimeError::ConditionTypeError(format!(
                //         "{}\nCondition evaluated to type and value of: {:?}",
                //         "Invalid condition value type, only Boolean values can be used as conditionals!",
                //         invalid_condition_type
                //     ))),
                // })?
            }

            Stmt::While(ref expr, ref loop_body) => {
                while self
                    .interpret_expr(expr)?
                    .bool_or_err("Expected Boolean from While loop expression")?
                {
                    // Execute stmt 1 by 1 and unwrap them with ? to allow any errors to stop execution and bubble up
                    self.interpret_stmt(loop_body)?;
                }
                None
            }

            // Constant definition statement, saves a Value into environment with the Const identifier as key
            Stmt::Const(ref token, ref expr) => {
                // Although the token definitely have a string lexeme if scanned correctly,
                // Rust treats this as a pattern matching context with a refutable pattern, so else case must be handled,
                // Which only happens if scanner failed to save String lexeme for Identifier type Token
                // Reference: https://stackoverflow.com/questions/41573764
                // Since this is a token of Identifier type, lexeme can be used directly
                if let Some(ref identifier) = token.lexeme {
                    // @todo This should be done in scanner/parser and not be a RuntimeError
                    // Check if the Const identifier has already been used in current scope
                    if self.env.borrow().in_current_scope(identifier) {
                        return Err(RuntimeError::ValueAlreadyDefined(identifier.clone()));
                    }

                    /*
                        self.env
                            .borrow_mut()
                            .define(identifier.to_string(), self.interpret_expr(expr)?);

                        Cannot compress code like above, because when running call to define method,
                        interpret_expr is ran first, and if the expr is a Expr::Const or something that accesses env,
                        with a borrow() or borrow_mut() then there will be a borrow error and panic.
                        Although it isn't very intuitive, since self.interpret_expr(expr)? is expected to run to completion
                        before control is handed over to the define method call, it will not work if chained.

                        // The above code will cause SS code on the Next line to fail with a
                        // thread 'main' panicked at 'already mutably borrowed: BorrowError', src\interpreter\interpreter.rs
                        const a = 1;
                        const b = a + 2; // Fails when accessing a value from env to assign to a new key in env
                    */
                    let expression = self.interpret_expr(expr)?;
                    self.env
                        .borrow_mut()
                        .define(identifier.to_string(), expression);
                    None
                } else {
                    // If somehow a identifier token does not have a string literal, then token Display trait is not helpful for debugging,
                    // Because it attempts to print out the string literal which is missing, thus print with debug symbol instead
                    return Err(RuntimeError::InternalError(format!(
                        "Runtime Error: Unable to set value on const identifier -> {:?}\n{}",
                        token, "Parsing error: Const identifier missing string literal\n"
                    )));
                }
            }

            Stmt::Print(ref expr) => {
                // Interpret expression and unwrap result to print
                // @todo Use seperate print from interpreter's print method, to make them run independently
                // @todo Dont just rely on println macro
                // @todo Right now only works for literal values if not using debug printing
                println!("{}", self.interpret_expr(expr)?);

                None
            }

            unmatched_stmt_variant => {
                return Err(RuntimeError::InternalError(format!(
                    "Failed to interpret statement.\nUnimplemented Stmt variant: {:#?}",
                    unmatched_stmt_variant
                )));
            }
        })
    }

    // Since interpreter might be passed into call method of callable trait, and it calls 'interpret_block',
    // which can modify the interpreter struct, a mutable ref to self (interpreter struct) is needed.
    fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            // @todo Optimize this
            // Right now this involves copying or cloning over a value from a Expr::Literal to Value:: variant
            // This can be removed by constructing Value variants directly in the parser, and wrapping them
            // in Expr::Literal. Then when interpreted, just return the Value variant within Expr::Literal.
            //
            // Using *Literal, to get the value from within the variant
            Expr::Literal(literal) => match *literal {
                Literal::Number(number) => Ok(Value::Number(number)),
                // Use a ref here to prevent moving it, and clone the string
                // @todo Move this instead of cloning it
                Literal::String(ref string) => Ok(Value::String(string.clone())),
                Literal::Bool(bool) => Ok(Value::Bool(bool)),
                Literal::Null => Ok(Value::Null),
            },

            // Expr::Call is a function call, which is an expression that evaluates to whatever the function returns
            // Checks to ensure the expression is a callable object
            // Evaluate and store all the arguments
            // Use callable.call and pass in the arguments
            //
            // This only takes care of checking the function expression callable part and arguments,
            // Calling/invoking/executing the function including creating a new scope is all taken care of in the call method
            // Which for user defined functions, is implemented in value::function module's Function struct's call method
            Expr::Call(ref callee, ref arguments, ref token) => {
                // Evaluate expression and ensure that the result is a callable function
                let callable = self.interpret_expr(callee)?.callable(token.line)?;

                // Create evaluated arguments list using length of arguments
                // @todo If supporting variadic functions or what not, then dont use with capacity since can change
                // @todo And also dont use it if we dont check for arity
                // @todo Arity should be checked for in parser too right?
                let mut evaluated_arguments: Vec<Value> = Vec::with_capacity(arguments.len());

                // @todo If following is JS, we will discard the extra arguments. Should we do this?
                for arg in arguments {
                    evaluated_arguments.push(self.interpret_expr(arg)?);
                }

                // Call function, either native or user defined using their common denominator, callable trait's call method
                callable.call(self, evaluated_arguments)
            }

            // Anonymous Functions are stored as an expression,
            // This expression when evaluated will create a new Function and return it within a Value::Func variant directly.
            // In essence Anonymous functions are implemented as Expressions that evaluate to a Value::Func type.
            //
            // This is usually called by Expr::Call's self.interpret_expr(callee) where it expects a Value::Func
            // The returned Value::Func is also stored in new function environment when this is used as a function argument
            Expr::AnonymousFunc(ref stmt) => Ok(Value::Func(Rc::new(Function::new(
                *stmt.clone(),
                Rc::clone(&self.env),
            )))),

            // A Const expression evaluates to the value stored in the environment identified by the Const's identifier
            // Distance is not implemented for now
            // @todo
            // Right now, all identifiers are parsed into Expr::Const variants, and the variant name does not adequately describe it
            // Because all Value identifiers, including Const and function identifiers are all parsed into Expr::Const
            // So unless we change this to Expr::Identifier / Expr::Value or we change the definition of Expr::Const, to one that points to all identifiers
            Expr::Const(ref token, ref distance) => {
                // Although the token definitely have a string lexeme if scanned correctly,
                // Rust treats this as a pattern matching context with a refutable pattern, so else case must be handled,
                // Which only happens if scanner failed to save String lexeme for Identifier type Token
                // Reference: https://stackoverflow.com/questions/41573764
                // Since this is a token of Identifier type, lexeme can be used directly
                if let Some(ref identifier) = token.lexeme {
                    // @todo
                    // Reference: https://stackoverflow.com/questions/30414424
                    // Should use get_ref here instead of get to avoid cloning the value
                    // But that would require changing the method's return type
                    //  Should Value even be moved out in the first place? Shouldnt all the values be immutable?
                    // Or perhaps return a mutable ref from env hashmap and every modification is made directly on the hashmap without needing additional update logic?
                    //
                    // Match is used here to convert error type from EnvError to RuntimeError, instead of using ? operator to bubble it up directly
                    match self.env.borrow().get(identifier, *distance) {
                        Ok(value) => Ok(value),
                        // @todo When not found, should it be an environment error or runtime error?
                        // Technically should be Runtime error, because it is caused by the user using a invalid identifier
                        // Environment errors are reserved for when there is a valid identifier but not found in environment
                        // Transform the error to RuntimeError --> This should be an internal problem right?
                        Err(e) => Err(RuntimeError::UndefinedIdentifier(
                            token.line,
                            identifier.clone(),
                        )),
                    }
                } else {
                    // Unlikely to happen because this will probably be caught by interpret_stmt's Const logic when setting a value
                    //
                    // If somehow a identifier token does not have a string literal, then token Display trait is not helpful for debugging,
                    // Because it attempts to print out the string literal which is missing, thus print with debug symbol instead
                    Err(RuntimeError::InternalError(format!(
                        "Runtime Error: Unable to read value on const identifier -> {:?}\n{}",
                        token, "Parsing error: Const identifier missing string literal\n"
                    )))
                }
            }

            Expr::ArrayAccess(ref array_identifier_expression, ref index_expression) => {
                // Evaluate expression into a Value enum variant, that should be Value::Array
                let array = self.interpret_expr(array_identifier_expression)?;

                // Evaluate expression into a Value enum variant, that should be Value::Number
                let index = self.interpret_expr(index_expression)?;

                // Check that the array is a Value::Array variant
                if let Value::Array(ref actual_array) = array {
                    // Check that the index is a Value::Number variant
                    if let Value::Number(ref index_number) = index {
                        // Since number is always f64 at least for now, it needs to be converted into usize to access an array
                        // Ref on using 'as' for casting: https://stackoverflow.com/a/28280042/13137262
                        let usize_index = *index_number as usize;

                        // Check if index is within bounds
                        // Since index_number is f64 type, it can be a negative number, thus need to ensure it is above the 0 lower limit
                        // Testing for 0 lower limit using index_number instead of usize_index as casting to usize will cause it to lose any neg. sign
                        // Test the array length upper limit using usize_index to ensure that it must be less than array length
                        if index_number >= &0.0 && usize_index < actual_array.len() {
                            // @todo Since value cannot be moved out of vec, element is cloned, alternative is to clone with Rc?
                            Ok(actual_array[usize_index].clone())
                        } else {
                            // @todo Find a way to include line number
                            Err(RuntimeError::ArrayOutOfBounds(format!(
                                "Array Index Out Of Bounds Error: Expect index to be 0 to {}, found -> {}",
                                actual_array.len() - 1,
                                index,
                            )))
                        }
                    } else {
                        // @todo Might want to add checks somehow in resolver to prevent this from being a runtime error
                        Err(RuntimeError::TypeError(format!(
                            "Array element access failed, expect index to be of type Value::Number, found -> {:?}",
                            index,
                        )))
                    }
                } else {
                    // @todo Might want to add checks somehow in resolver to prevent this from being a runtime error
                    Err(RuntimeError::TypeError(format!(
                        "Array element access failed, expect array to be of type Value::Array, found -> {:?}",
                        array,
                    )))
                }
            }

            Expr::Array(_, ref element_expressions) => {
                // Create elements value vector using length of element expressions
                let mut elements: Vec<Value> = Vec::with_capacity(element_expressions.len());
                for element in element_expressions {
                    elements.push(self.interpret_expr(element)?);
                }
                Ok(Value::Array(elements))
            }

            Expr::Grouping(ref expr) => self.interpret_expr(expr),

            Expr::Unary(ref token, ref expr) => {
                let value = self.interpret_expr(expr)?;

                match &token.token_type {
                    TokenType::Minus => match value {
                        Value::Number(number) => Ok(Value::Number(-number)),
                        _ => Err(RuntimeError::TypeError(
                            // "Invalid types used for number negation!",
                            "Invalid types used for number negation!".to_string(),
                        )),
                    },

                    // Should this support other types, smth like this to handle types? https://stackoverflow.com/a/59152263/13137262
                    TokenType::Bang => match value {
                        Value::Bool(bool) => Ok(Value::Bool(!bool)),
                        _ => Err(RuntimeError::TypeError(
                            // "Invalid types used for boolean negation!",
                            "Invalid types used for boolean negation!".to_string(),
                        )),
                    },

                    // @todo Not needed as case is covered in TypeChecker
                    operator => Err(RuntimeError::InternalError(format!(
                        "Invalid unary operator: {:?}",
                        operator
                    ))),
                }
            }

            // Binary expression with an operator and 2 operands
            Expr::Binary(ref left, ref operator, ref right) => {
                // This evaluates the Binary expression from left to right
                // @todo Might want to change this for certain cases to support bool short circuiting
                let left_value = self.interpret_expr(left)?;
                let right_value = self.interpret_expr(right)?;

                match &operator.token_type {
                    TokenType::Plus => {
                        // arithmetic_binary_op!(+, left_value, right_value, "Invalid types used for addition!")
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Number(left_number + right_number))
                            }
                            // Overloading the + operator to support string concatenation
                            (Value::String(left_string), Value::String(right_string)) => {
                                // @todo Choose a way for string concat
                                // Ok(Value::String(format!("{}{}", left_string, right_string)))
                                Ok(Value::String(left_string + &right_string))
                            }
                            // Change all unmatched patterns to this pattern, so that we can use it to show the mismatched types
                            // (left_value, right_value)  => Err(RuntimeError::TypeError(
                            _ => Err(RuntimeError::TypeError(
                                // @todo Show types used
                                // "Invalid types used for addition!",
                                "Invalid types used for addition!".to_string(),
                            )),
                        }
                    }

                    TokenType::Minus => {
                        arithmetic_binary_op!(-, left_value, right_value, "Invalid types used for subtraction!")
                    }

                    TokenType::Star => {
                        arithmetic_binary_op!(*, left_value, right_value, "Invalid types used for multiplication!")
                    }

                    TokenType::Slash => {
                        arithmetic_binary_op!(/, left_value, right_value, "Invalid types used for division!")
                    }

                    // @todo Can we add a try/catch? Then if fail, return the Err(InternalError or TypeError for cannot compare)
                    // @todo Allows for comparison of primitive types and string so far, but might want to test for complex types like Functions
                    // @todo Once type inference is done, only allow equality checks between item of same time, else panic
                    TokenType::EqualEqual => {
                        // Can do direct comparison here as long as Value enum derives the PartialEq trait
                        Ok(Value::Bool(left_value == right_value))
                    }
                    TokenType::BangEqual => {
                        // Can do direct comparison here as long as Value enum derives the PartialEq trait
                        Ok(Value::Bool(left_value != right_value))
                    }

                    TokenType::Greater => {
                        numeric_comparison_op!(>, left_value, right_value, "Invalid types used for Greater!")
                    }

                    TokenType::GreaterEqual => {
                        numeric_comparison_op!(>=, left_value, right_value, "Invalid types used for Greater Equal!")
                    }

                    TokenType::Less => {
                        numeric_comparison_op!(<, left_value, right_value, "Invalid types used for Less!")
                    }

                    TokenType::LessEqual => {
                        numeric_comparison_op!(<=, left_value, right_value, "Invalid types used for Less Equal!")
                    }

                    unmatched_token_type => {
                        panic!(
                            "Internal Error: Invalid binary operator {:?}",
                            unmatched_token_type
                        )
                    }
                }
            }

            // Support for truthy operations, with returning values
            // Expr::Logical(ref left, ref operator, ref right) => {
            //     let left_value = self.interpret_expr(left)?;
            //
            //     if operator.token_type == TokenType::Or {
            //         if left_value.is_truthy() {
            //             return Ok(left_value);
            //         }
            //     } else if operator.token_type == TokenType::And {
            //         if !left_value.is_truthy() {
            //             return Ok(Value::Bool(false));
            //         }
            //     } else {
            //         return Err(RuntimeError::InternalError(format!(
            //             "Parsing Error: Invalid Token Type for logical expr -> {:?}",
            //             operator.token_type
            //         )));
            //     }
            //     self.interpret_expr(right)
            // }
            // Strict boolean operator evaluation, without truthy operations, evaluates to a Bool Value
            Expr::Logical(ref left_expr, ref operator, ref right_expr) => {
                let left_value = self.interpret_expr(left_expr)?;

                if operator.token_type == TokenType::Or {
                    // If left value is boolean true, ignore right expression and short circuit to true
                    // Else, interpret right expression and use is_bool_true method to return boolean value
                    Ok(Value::Bool(
                        if left_value.bool_or_err("Logical operations only work with Bool Types")? {
                            true
                        } else {
                            self.interpret_expr(right_expr)?
                                .bool_or_err("Logical operations only work with Bool Types")?
                        },
                    ))
                } else if operator.token_type == TokenType::And {
                    // If left value is boolean false, ignore right expression and short circuit to false
                    // Else, interpret right expression and use is_bool_true method to return boolean value
                    Ok(Value::Bool(
                        if left_value.bool_or_err("Logical operations only work with Bool Types")? {
                            self.interpret_expr(right_expr)?
                                .bool_or_err("Logical operations only work with Bool Types")?
                        } else {
                            false
                        },
                    ))
                } else {
                    // Unlikely to happen, but if somehow a logical expression does not have a valid token_type,
                    // Then it is an internal error caused by the parser
                    Err(RuntimeError::InternalError(format!(
                        "Parsing Error: Invalid Token Type for logical expr -> {:?}",
                        operator.token_type
                    )))
                }
            }

            #[allow(unreachable_patterns)]
            unmatched => Err(RuntimeError::InternalError(format!(
                "Unimplemented expr type -> {}",
                unmatched
            ))),
        }
    }
}
