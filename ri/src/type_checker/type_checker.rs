use super::error::TypeError;
use super::type_table::TypeTable;
use super::Type;
use super::TypeChecker;

use std::cell::RefCell;
use std::rc::Rc;

use crate::literal::Literal;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

/*
    Unused values checked here instead of resolver to piggy back the logic as its easier to just make sure there is no "return" types left
*/
impl TypeChecker {
    // Associated function to type check a AST (where AST in this case is a vec of Stmt variants)
    pub fn check(ast: &Vec<Stmt>) -> Result<(), TypeError> {
        // Create TypeChecker instance internally
        let mut type_checker = TypeChecker {
            // Create global type table with the types of global values pre-defined in the method
            env: Rc::new(RefCell::new(TypeTable::global())),

            // No closure applicable for top level code
            closure_types: None,

            // Global scope is not a function unlike languages like C where there is a main function as entry point
            current_function: None,
        };

        // @todo Add a synchronization method, to prevent type checker from quitting on first error, and instead, check other errors and return all via an array
        type_checker.check_ast(ast)?;

        Ok(())
    }

    /// Type check statements 1 by 1 by iterating through the vec of statements instead of calling this recursively for efficiency
    fn check_ast(&mut self, ast: &Vec<Stmt>) -> Result<Type, TypeError> {
        for ref stmt in ast {
            let stmt_type = self.check_statement(stmt)?;
            if let Type::Return(_) = stmt_type {
                // Stop and bubble up stmt_type if Type::Return, to bubble through everything and let function checker handle it
                return Ok(stmt_type);
            }

            // Checks for unused values to ensure that there no values are left unused
            match stmt_type {
                // Ignore types that are allowed to be "unused" in global scope
                Type::Func(_, _, _) | Type::None => {}

                // Expressions (including anonymous functions) types are unused values
                // @todo Get line number from stmt for error reporting
                value_type => return Err(TypeError::UnusedValue(value_type)),
            }
        }

        // Default type of an AST as this does not evaluate to any value by default, thus it DOES NOT HAVE a value type.
        // Only return statements and statements with nested return statements can have a type to bubble up.
        Ok(Type::None)
    }

    // Type check a given statement, and return the statement's inferred type if any
    fn check_statement(&mut self, stmt: &Stmt) -> Result<Type, TypeError> {
        // Any stmt that resolves into a Type, will have to manually return it
        match *stmt {
            Stmt::Expr(ref expr) => {
                // @todo Why need to return here? Is it because a return stmt can be nested?
                return self.check_expression(expr);
            }
            Stmt::Block(ref stmts, _) => {
                // Get a new Rc<TypeTable> pointing to the same TypeTable in memory
                // Essentially, get a reference to self.env by cloning a pointer to it and not actually clone the underlying data
                let parent_env = Rc::clone(&self.env);

                // Create new type table for current function block with existing type table as the enclosing one
                let current_env = TypeTable::new(Some(Rc::clone(&self.env)));

                // Set the new type table directly onto struct so other methods can access it directly
                // @todo Can be better written, by changing all the methods to take current scope as function argument,
                // @todo instead of saving current environment temporarily and attaching the new environment to self.
                self.env = Rc::new(RefCell::new(current_env));

                /*  */

                // Store block stmt type to type check for return types after ending current scope
                let block_stmt_type = self.check_ast(stmts)?;

                // Reset parent environment back onto the struct once block completes execution
                // The newly created current environment for this block will be dropped once function exits
                self.env = parent_env;

                // @todo Is this needed or can just bubble up since check_function also matches for Type::Return
                // Bubble up block_stmt_type if it is Type::Return to let function checker handle it
                if let Type::Return(_) = block_stmt_type {
                    return Ok(block_stmt_type);
                }
            }
            Stmt::Const(ref identifier_token, ref expr) => {
                // Must be split to prevent borrow as mutable when also borrowed as immutable
                let expr_type = self.check_expression(expr)?;
                self.env
                    .borrow_mut()
                    .define(identifier_token.lexeme.as_ref().unwrap().clone(), expr_type);
            }
            Stmt::Func(ref identifier_token, ref params, ref body) => {
                // Clone function stmt to store as part of the Type::Func(..) to type check the function again when it's called.
                // Type check again when called, with the arguments' types mapped to the parameter identifiers
                // Num of params is stored to ensure number of arguments matches in function call.
                // Get closure values by 'cloning' current type table by getting a ref to it, similiar to how value/function.rs stores closure
                let function_type =
                    Type::Func(params.len(), Box::new(stmt.clone()), Rc::clone(&self.env));

                let identifier_string = identifier_token.lexeme.as_ref().unwrap();

                // Add function to type table before type checking function body to allow function to refer to itself recursively.
                self.env
                    .borrow_mut()
                    .define(identifier_string.clone(), function_type.clone());

                // Pass in the identifier token since this is a named function and can be used recursively
                //
                // Param types are not available as no type annotations thus all param types are "generic" during function definition,
                //
                // Method will return function's return type IF it is able to resolve any, else defaults to Type::None
                // HOWEVER, return type is not used since it's only needed when type checking function call expressions to determine their types
                self.check_function(Some(identifier_token), params, None, body)?;

                // Return function_type as the type of this function definition
                return Ok(function_type);
            }
            Stmt::AnonymousFunc(ref params, ref body) => {
                // Clone function stmt to store as part of the Type::Func(..) to type check the function again when it's called.
                // Type check again when called, with the arguments' types mapped to the parameter identifiers
                // Num of params is stored to ensure number of arguments matches in function call.
                // Get closure values by 'cloning' current type table by getting a ref to it, similiar to how value/function.rs stores closure
                let function_type =
                    Type::AnonymousFunc(params.len(), Box::new(stmt.clone()), Rc::clone(&self.env));

                // Anonymous functions cannot do recursion by referencing identifier this Expr::AnonymousFunc is binded to,
                // as function type is not added to type table before type checking function body thus cannot refer to itself recursively
                // @todo Languages like F# deals with this using a rec keyword if it can be used recursively
                // @todo Alternatively make it an error to refer to itself recursively if it is an anonymous function in parser

                // No identifier token since this is an anonymous function that cannot be used recursively
                //
                // Param types are not available as no type annotations thus all param types are "generic" during function definition,
                //
                // Method will return function's return type IF it is able to resolve any, else defaults to Type::None
                // HOWEVER, return type is not used since it's only needed when type checking function call expressions to determine their types
                self.check_function(None, params, None, body)?;

                // Return function_type as the type of this function definition
                return Ok(function_type);
            }
            Stmt::If(ref condition, ref then_branch, ref else_branch, _) => {
                if self.check_expression(condition)? != Type::Bool {
                    return Err(TypeError::InternalError(
                        "TESTING - Conditions of If stmts must be bool",
                    ));
                }

                // Assuming most if/else blocks either have return stmts within both bodies or none.
                let mut return_types = Vec::<Type>::with_capacity(2);

                let if_body_type = self.check_statement(then_branch)?;
                if let Type::Return(_) = if_body_type {
                    // Push return type instead of unwrapping for inner type as it's handled by function call type checker
                    return_types.push(if_body_type);
                }

                // Only type check else branch if there is an else branch
                if let Some(ref else_branch) = else_branch {
                    let else_body_type = self.check_statement(else_branch)?;
                    if let Type::Return(_) = else_body_type {
                        // Push return type instead of unwrapping for inner type as it's handled by function call type checker
                        return_types.push(else_body_type);
                    }
                }

                // Type check return values, and bubble them up if any
                if !return_types.is_empty() {
                    return Ok(if return_types.len() == 1 {
                        // If there is only a single return, use the type immediately without further checks
                        // Move out from vec since vec is no longer needed
                        return_types.remove(0)
                    } else {
                        // Loop only useful if there is more than one if/else, i.e if there are any 'else-if' blocks
                        for return_type in &return_types {
                            if return_type != &return_types[0] {
                                return Err(TypeError::InternalError(
                                    "TESTING - Function must have the same return type throughout the function body"
                                ));
                            }
                        }
                        // If all return types are the same, then move out and use first type as function return type
                        return_types.remove(0)
                    });
                }
            }
            Stmt::Print(ref expr) => {
                // This cannot be skipped because even though print accepts all types, the expression needs to be type checked first
                // E.g. the expression can be a 5 == "string", and this needs to be checked, even if the expression type is unused
                // The type of the expression is ignored, but the ? operator is used to allow errors to bubble up
                self.check_expression(expr)?;
            }
            Stmt::Return(ref expr, _) => {
                // Get the type of the return expression,
                // Wrap it in a Return type and Ok variant to bubble it up
                return Ok(Type::Return(Box::new(self.check_expression(expr)?)));
            }
            // Ignore statements are used to ignore evaluated values of expressions,
            // Which is done by type checking the expression and only bubbling up errors if needed.
            Stmt::Ignore(ref expr) => {
                self.check_expression(expr)?;
            }
            Stmt::While(ref condition, ref body, _) => {
                return match self.check_expression(condition)? {
                    // If there are any return statements within loop, the type will be bubbled up.
                    Type::Bool => self.check_statement(body),

                    // Only Bools can be used for loop condition
                    unexpected_type => Err(TypeError::InternalError(
                        "Expect boolean condition for While statements, found 'unexpected_type'",
                    )),
                };
            }
        };

        // Statements do not evaluate to any value by default, thus they DO NOT HAVE a value type.
        // Only return statements and statements with nested return statements can have a type to bubble up.
        Ok(Type::None)
    }

    // Type check a given expression, and return the expression's inferred type
    fn check_expression(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        Ok(match *expr {
            Expr::Const(ref token, _) => {
                // Use lexeme from token as identifier
                let identifier_string = token.lexeme.as_ref().unwrap();

                // Once file has been resolved all identifiers are checked to have been defined before use, which means
                // If types are inserted into type table correctly, type of identifier MUST exist in current environment or closure
                // Therefore if type not found, it is a internal type table programming logic error

                // @todo Look for type in closure first or current env first?
                if let Some(value_type) = self.env.borrow().get_type(identifier_string) {
                    value_type
                } else if let Some(ref closure_types) = self.closure_types {
                    if let Some(value_type) = closure_types.borrow().get_type(identifier_string) {
                        value_type
                    } else {
                        panic!(
                            "InternalError: Type of '{}' is not found in both current environment and closure",
                            identifier_string
                        );
                    }
                } else {
                    panic!(
                        "InternalError: Type of '{}' is not found in current environment and there is no closure environment",
                        identifier_string
                    );
                }
            }

            // Expr::AnonymousFunc is a wrapper for Stmt::AnonymousFunc, thus use check_statement to handle Stmt::AnonymousFunc
            Expr::AnonymousFunc(ref stmt) => self.check_statement(stmt)?,

            // @todo Add new arithmetic expr to split this up, so that the operator check can be skipped
            // Binary expressions holds both equality/inequality checks, and arithmetic operations
            Expr::Binary(ref left, ref operator, ref right) => {
                let l_type = self.check_expression(left)?;
                let r_type = self.check_expression(right)?;

                // Regardless of their types, operands of binary expressions must always have the SAME type
                if l_type == r_type {
                    // Return a type based on the binary operator
                    match &operator.token_type {
                        // Comparison expressions allow operands to be of any types as long as they are the same,
                        // And will always be evaluated to a value of Type::Bool
                        TokenType::EqualEqual | TokenType::BangEqual => Type::Bool,

                        // Arithmetic expressions ONLY ALLOW Type::Number operands,
                        // And will always be evaluated to a value of Type::Number
                        // @todo Might need to change handling of arithmetic operators if supporting different number types like unsigned int
                        // @todo Change Plus type to allow strings, as Plus operator is overloaded to support string concat in the interpreter
                        TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Star => {
                            if l_type == Type::Number {
                                Type::Number
                            } else {
                                return Err(TypeError::InternalError(
                                    "TESTING - Binary - Expect number for '&operator.token_type'",
                                ));
                            }
                        }

                        // Numeric comparison expressions ONLY ALLOW Type::Number operands,
                        // And will always be evaluated to a value of Type::Bool
                        TokenType::Greater
                        | TokenType::GreaterEqual
                        | TokenType::Less
                        | TokenType::LessEqual => {
                            if l_type == Type::Number {
                                Type::Bool
                            } else {
                                return Err(TypeError::InternalError(
                                    "TESTING - Binary - Expect number for '&operator.token_type'",
                                ));
                            }
                        }

                        unmatched_token_type => {
                            panic!("Internal Error: Invalid token_type {:?} found in Expr::Binary of TypeChecker", unmatched_token_type)
                        }
                    }
                } else {
                    // @todo Fix error msg, add a, 'found type {l_type} and {r_type}'
                    return Err(TypeError::InternalError(
                        "TESTING - Binary - operands of binary expressions must have the SAME type",
                    ));
                }
            }

            /*  How Expr::Call is type checked:
                ---
                Type check callee_identifier_expr, and get back function type,
                to type check the function again with the types of the arguments for THIS CALL
                Which means to say a function can be used with multiple types of arguments,
                as long as they can pass the type check for each instance of function call.
                Which means this allows for safe generics without any explicit annotations
                ---

                Example 1: In this example the use of any type is permitted because print accepts all types
                fn test(val) {
                    print val; // Print accepts values of any type
                }
                // Both works since both arguments, when used as the function's parameter type passes the type check
                test(1);
                test("string");


                Example 2: In this example the use of any types is permitted, AS LONG AS the given types are the same
                fn check(a, b) {
                    return a == b;
                }
                check(1, 1);
                check("s1", "s2");
            */
            Expr::Call(ref callee_identifier_expr, ref arguments, _) => {
                // If this resolves to a valid Type::Func(..), then extract the tuple's value.
                let (number_of_parameters, function_stmt, closure_types) =
                    match self.check_expression(callee_identifier_expr)? {
                        Type::Func(number_of_parameters, function_stmt, closure_types) => {
                            (number_of_parameters, function_stmt, closure_types)
                        }
                        Type::AnonymousFunc(number_of_parameters, function_stmt, closure_types) => {
                            (number_of_parameters, function_stmt, closure_types)
                        }

                        value_type => {
                            // @todo fix error and show the actual value type used
                            return Err(TypeError::InternalError(
                                "TESTING - cannot call 'value_type' as a function",
                            ));
                        }
                    };

                // Ensure that the number of arguments matches the number of parameters defined
                if arguments.len() != number_of_parameters {
                    return Err(TypeError::InternalError(
                        "TESTING - Call - different numbers of arguments",
                    ));
                }

                // Create a fixed length vec of arg types and get the arg types by resolving the args individually
                let mut argument_types: Vec<Type> = Vec::with_capacity(arguments.len());
                for ref arg in arguments {
                    argument_types.push(self.check_expression(arg)?);
                }

                // Get items needed to type check function from the Function's AST node
                let (optional_identifier_token, param_tokens, argument_types, body) =
                    match *function_stmt {
                        Stmt::Func(ref identifier_token, ref params, ref body) => {
                            (Some(identifier_token), params, Some(argument_types), body)
                        }
                        Stmt::AnonymousFunc(ref params, ref body) => {
                            (None, params, Some(argument_types), body)
                        }
                        _ => panic!("Internal Error: Expected Func type stmt body in Type::Func"),
                    };

                // Store closure type on TypeChecker before type checking function so if needed Expr::Const(..) logic can access it
                // @todo If it is more than 1 layer of nesting, the previous self.closure_types will get overwritten
                self.closure_types = Some(closure_types);

                // Type check the function again, this time with the types of the arguments as types of the parameters
                // The type of the call expression is the return type of the function called after resolving it
                self.check_function(
                    optional_identifier_token,
                    param_tokens,
                    argument_types,
                    body,
                )?
            }
            Expr::Grouping(ref expr) => self.check_expression(expr)?,
            Expr::Literal(ref literal) => match literal {
                // @todo Might need to split into signed and unsigned num
                Literal::Number(_) => Type::Number,
                Literal::String(_) => Type::String,
                Literal::Bool(_) => Type::Bool,
                // @todo Are null types still needed now that Type::None exists?
                Literal::Null => Type::Null,
            },
            Expr::Array(_, ref elements) => {
                let array_element_type = self.check_expression(&elements[0])?;

                // Resolve for elements[1..] of the array, where all elements are expressions
                for element in elements.into_iter().skip(1) {
                    if self.check_expression(element)? != array_element_type {
                        return Err(TypeError::InternalError("TESTING - Array"));
                    }
                }

                Type::Array(Box::new(array_element_type))
            }
            Expr::ArrayAccess(ref array_identifier_expr, ref index_expression) => {
                // @todo Ensure that the indexing expression is a unsigned integer, not just a number, to remove the runtime check
                if self.check_expression(index_expression)? != Type::Number {
                    return Err(TypeError::InternalError(
                        "TESTING - index expression must be uint",
                    ));
                }

                // If this resolves to a valid Type::Array(..) type, then extract the 'array_element_type'
                match self.check_expression(array_identifier_expr)? {
                    Type::Array(array_element_type) => *array_element_type,

                    value_type => {
                        // @todo fix error and show the actual value type used
                        return Err(TypeError::InternalError(
                            "TESTING - cannot access 'value_type' as an array",
                        ));
                    }
                }
            }
            Expr::Logical(ref left, _, ref right) => {
                let l_type = self.check_expression(left)?;
                let r_type = self.check_expression(right)?;

                // Both operand types in Logical expressions must be Type::Bool and always evaluate to a Boolean value
                match (l_type, r_type) {
                    (Type::Bool, Type::Bool) => Type::Bool,
                    _ => {
                        return Err(TypeError::InternalError(
                            "TESTING - Logical expressions must be bool",
                        ))
                    }
                }
            }
            Expr::Unary(ref operator, ref expr) => {
                let expr_type = self.check_expression(expr)?;

                match &operator.token_type {
                    TokenType::Bang => {
                        if expr_type == Type::Bool {
                            Type::Bool
                        } else {
                            return Err(TypeError::InternalError(
                                "TESTING - Unary NOT expressions must be bool",
                            ));
                        }
                    }
                    TokenType::Minus => {
                        if expr_type == Type::Number {
                            Type::Number
                        } else {
                            return Err(TypeError::InternalError(
                                "TESTING - Unary NEGATE expressions must be Number",
                            ));
                        }
                    }
                    invalid_token_type => panic!(
                        "Internal Error: Found {:?} in Expr::Unary",
                        invalid_token_type
                    ),
                }
            }
        })
    }

    /// Arguments:
    /// - `optional_identifier_token`: Optional identifier token used to prevent infinite recursion when type checking recursive function calls of named functions
    /// - `param_tokens`: The parameters' token
    /// - `argument_types`: Optional vec of types mapped to the param_tokens vec to type check, if None, they will be type checked as generics
    /// - `body`: The body of the function statement to type check
    ///
    /// Return:
    /// - Returns the return type of the function if any return type can be determined else defaults to Type::None
    fn check_function(
        &mut self,
        optional_identifier_token: Option<&Token>,
        param_tokens: &Vec<Token>,
        argument_types: Option<Vec<Type>>,
        body: &Stmt,
    ) -> Result<Type, TypeError> {
        // Save parent function's name first if any before assigning the name of the current function
        let parent_identifier_token = self.current_function.clone();

        self.current_function = match optional_identifier_token {
            Some(identifier_token) => {
                // Check if the function is a recursive one, by checking if the name of the function called is the same as the parent function
                // Regardless if type checking function definition or a call to the function, always return Type::Lazy to prevent infinite recursive type checking
                // @todo Type::Lazy is used to match any type, to test if this is sound..
                if let Some(ref parent_identifier_token) = self.current_function {
                    if parent_identifier_token == identifier_token {
                        return Ok(Type::Lazy);
                    }
                }

                Some(identifier_token.clone())
            }
            None => None,
        };

        // Get a new Rc<TypeTable> pointing to the same TypeTable in memory
        // Essentially, get a reference to self.env by cloning a pointer to it and not actually clone the underlying data
        let parent_env = Rc::clone(&self.env);

        // Create new type table for current function block with existing type table as the enclosing one
        let current_env = TypeTable::new(Some(Rc::clone(&self.env)));

        // Set the new type table directly onto struct so other methods can access it directly
        // @todo Can be better written, by changing all the methods to take current scope as function argument,
        // @todo instead of saving current environment temporarily and attaching the new environment to self.
        self.env = Rc::new(RefCell::new(current_env));

        // Hard to merge with closures, thus 2 seperate loop
        match argument_types {
            // If argument types are given (type checking function call), use them to type check function body
            Some(mut argument_types) => {
                let mut scope = self.env.borrow_mut();

                for param_token in param_tokens {
                    // scope.insert(
                    scope.define(
                        param_token.lexeme.as_ref().unwrap().clone(),
                        // Remove instead of cloning as vec is no longer needed after this operation
                        // Always remove the first element, since after each remove all elements will be shifted left
                        argument_types.remove(0),
                    );
                }
            }
            // If argument types not given (type checking function definition), use Type::Lazy to defer some type checking
            None => {
                let mut scope = self.env.borrow_mut();

                for param_token in param_tokens {
                    // Save type of every parameter into scope as Type::Lazy during this function definition type checking process,
                    // To defer type checking for statements that use these parameters till function call type checks,
                    // And during which the type of the arguments will be available
                    // scope.insert(param_token.lexeme.as_ref().unwrap().clone(), Type::Lazy);
                    scope.define(param_token.lexeme.as_ref().unwrap().clone(), Type::Lazy);
                }
            }
        }

        // Assuming most functions only have 1 return statement
        let mut return_types = Vec::<Type>::with_capacity(1);

        // Body must be a block statement, even for anonymous arrow functions
        // arrow functions is just syntatic sugar and are also parsed into block statements
        if let &Stmt::Block(ref stmts, _) = body {
            for stmt in stmts {
                // Destructure out the inner type and push onto return_types array to do return type, type checking later.
                // Return types are usually bubbled up in block statements to let the function call type checking method handle it,
                // And since this check_function method is the highest level a return_type should be bubbled up to, it is unwrapped here.
                if let Type::Return(return_type) = self.check_statement(stmt)? {
                    return_types.push(*return_type);
                }
            }
        } else {
            panic!("Internal Error: Function body can only be Stmt::Block");
        };

        // Reset parent environment back onto the struct once block completes execution
        // The newly created current environment for this block will be dropped once function exits
        self.env = parent_env;

        // Restore the parent function's identifier token now that the call has been type checked
        self.current_function = parent_identifier_token;

        Ok(
            // If there are no return statements, default return type is None
            if return_types.is_empty() {
                Type::None
            } else if return_types.len() == 1 {
                // If there is only a single return, use the type immediately without further checks
                // Move out from vec since vec is no longer needed
                return_types.remove(0)
            } else {
                // @todo Optimize by skipping the first element, otherwise it will be compared with itself
                for return_type in &return_types {
                    if return_type != &return_types[0] {
                        return Err(TypeError::InternalError(
                            "TESTING - Function must have the same return type throughout the function body"
                        ));
                    }
                }

                // If all return types are the same, then move out first type as function return type
                return_types.remove(0)
            },
        )
    }
}
