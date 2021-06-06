use super::error::TypeCheckerError;
use super::Type;
use super::TypeChecker;

use crate::literal::Literal;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

impl TypeChecker {
    // Associated function to resolve a AST
    pub fn check(ast: &Vec<Stmt>) -> Result<(), TypeCheckerError> {
        // Create TypeChecker instance internally
        let mut type_checker = TypeChecker {
            scopes: Vec::new(),
            current_function: None,

            // @todo A better way other than hardcoding all identifiers in
            globals: vec!["clock"],
        };

        // Create first new scope for the global scope and insert in identifiers
        type_checker.begin_scope();

        // @todo Make it better then.. Cloning it because cannot have ref and mut ref to type_checker at the same time....
        type_checker.define_globals(type_checker.globals.clone());

        // @todo Add a synchronization method, to prevent type checker from quitting on first error, and instead, check other errors and return all via an array
        type_checker.resolve_ast(ast)?;
        type_checker.end_scope();

        Ok(())
    }

    /// resolve_ast(Vec_of_stmts)
    ///
    /// Resolve statements 1 by 1 by iterating through the vec of statements instead of calling this recursively for efficiency
    fn resolve_ast(&mut self, ast: &Vec<Stmt>) -> Result<Type, TypeCheckerError> {
        for ref stmt in ast {
            let stmt_type = self.resolve_statement(stmt)?;
            if let Type::Return(_) = stmt_type {
                // Stop and bubble up stmt_type if Type::Return, to bubble through everything and let function checker handle it
                return Ok(stmt_type);
            }
        }

        // Default type of the statement
        // Change to a void type or something
        Ok(Type::Null)
    }

    // @todo Use reference to the string instead of having to own it for lexeme.clone()
    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<Type, TypeCheckerError> {
        // Any stmt that resolves into a Type, will have to manually return it
        match *stmt {
            Stmt::Expr(ref expr) => {
                // @todo Why need to return here? Is it because a return stmt can be nested?
                return self.resolve_expression(expr);
            }
            Stmt::Block(ref stmts) => {
                self.begin_scope();
                // @todo Handle returns
                self.resolve_ast(stmts)?;
                self.end_scope();
            }
            Stmt::Const(ref identifier_token, ref expr) => {
                let expr_type = self.resolve_expression(expr)?;

                // Save type of expression into scope using the identifier_token's lexeme as key
                // - A scope is always expected to exists, including the global top level scope
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(identifier_token.lexeme.as_ref().unwrap().clone(), expr_type);
            }
            Stmt::Func(ref identifier_token, ref params, ref body) => {
                // Clone the function stmt as it will be stored as part of the Type::Func(..),
                // So that it can be used to type check the function again during a function call,
                // When types are available for the parameters by using the types of the arguments.
                let function_stmt = stmt.clone();

                let identifier_string = identifier_token.lexeme.as_ref().unwrap();

                // Add function to scope before type checking function body to allow function to refer to itself recursively.
                // Since function body is not type checked yet, Return type is unknown therefore this uses Type::Lazy as a placeholder
                // Num of params is stored to ensure that the number of arguments matches it, instead of param types since there isn't any
                self.scopes.last_mut().unwrap().insert(
                    identifier_string.clone(),
                    Type::Func(
                        params.len(),
                        Box::new(Type::Lazy),
                        Box::new(function_stmt.clone()),
                    ),
                );

                // Call resolve function to continue type checking function body with Type::Lazy for the parameters
                // Method will return the function's return type IF it is able to resolve it
                let return_type = self.resolve_function(identifier_string.clone(), params, body)?;

                // Create function type again, this time hopefully with a return type, update scope and return this
                let function_type =
                    Type::Func(params.len(), Box::new(return_type), Box::new(function_stmt));
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(identifier_string.clone(), function_type.clone());
                return Ok(function_type);
            }
            // Stmt::AnonymousFunc(ref params, ref body) => {
            //     // Unlike Stmt::Func, dont need to declare and define since Anonymous Functions are nameless, and will be bound to a Const identifier
            //     self.resolve_function(params, body)?;
            // }
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                if self.resolve_expression(condition)? != Type::Bool {
                    return Err(TypeCheckerError::InternalError(
                        "TESTING - Conditions of If stmts must be bool",
                    ));
                }

                self.resolve_statement(then_branch)?;

                // Only type check else branch if it exists
                if let Some(ref else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Stmt::Print(ref expr) => {
                // This cannot be skipped because even though print accepts all types, the expression needs to be type checked first
                // E.g. the expression can be a 5 == "string", and this needs to be checked, even if the Bool type returned can be ignored
                // The type returned will be ignored, but the ? operator is used to allow errors to bubble up
                self.resolve_expression(expr)?;
            }
            Stmt::While(ref condition, ref body) => {
                if self.resolve_expression(condition)? != Type::Bool {
                    return Err(TypeCheckerError::InternalError(
                        "Expect boolean condition for While statements",
                    ));
                }

                // @todo Check if this is a return Type, if so bubble up...
                self.resolve_statement(body)?;
            }
            Stmt::Return(ref token, ref expr) => {
                // Get the type of the return expression,
                // Wrap it in a Return type, and Ok variant to bubble it up
                return Ok(Type::Return(Box::new(self.resolve_expression(expr)?)));
            }

            #[allow(unreachable_patterns)]
            ref unmatched_stmt_variant => panic!("{}", unmatched_stmt_variant),
        };

        // Default type of the statement
        // Change to a void type or something
        Ok(Type::Null)
    }

    fn resolve_expression(&mut self, expr: &Expr) -> Result<Type, TypeCheckerError> {
        Ok(match *expr {
            Expr::Const(ref token, _) => self.get_type(token),

            // Expr::AnonymousFunc(ref stmt) => {
            //     // Expr::AnonymousFunc is a wrapper for Stmt::AnonymousFunc, thus use resolve_statement to handle Stmt::AnonymousFunc
            //     self.resolve_statement(stmt)?;
            // }

            // @todo Add new arithmetic expr to split this up, so that the operator check can be skipped
            // Binary expressions holds both equality/inequality checks, and arithmetic operations
            Expr::Binary(ref left, ref operator, ref right) => {
                let l_type = self.resolve_expression(left)?;
                let r_type = self.resolve_expression(right)?;

                // Regardless of their types, operands of binary expressions must always have the SAME type
                if l_type == r_type {
                    // Return a type based on the binary operator
                    match &operator.token_type {
                        // Comparison expressions allow operands to be of any types as long as they are the same,
                        // And will always be evaluated to a value of Type::Bool
                        TokenType::EqualEqual | TokenType::BangEqual => Type::Bool,

                        // Arithmetic and Numeric comparison expressions ONLY ALLOW Type::Number operands,
                        // And will always be evaluated to a value of Type::Number
                        // @todo Might need to change handling of arithmetic operators if supporting different number types like unsigned int
                        // @todo Change Plus type to allow strings, as Plus operator is overloaded to support string concat in the interpreter
                        TokenType::Plus
                        | TokenType::Minus
                        | TokenType::Slash
                        | TokenType::Star
                        | TokenType::Greater
                        | TokenType::GreaterEqual
                        | TokenType::Less
                        | TokenType::LessEqual => {
                            if l_type == Type::Number {
                                Type::Number
                            } else {
                                return Err(TypeCheckerError::InternalError(
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
                    return Err(TypeCheckerError::InternalError(
                        "TESTING - Binary - operands of binary expressions must have the SAME type",
                    ));
                }
            }
            Expr::Call(ref callee_identifier_expr, ref arguments, _) => {
                /*
                    Type check callee_identifier_expr, and get back function type,
                    to type check the function again with the types of the arguments for THIS CALL
                    Which means to say a function can be used with multiple types of arguments,
                    as long as they can pass the type check for each instance of function call.
                    Which means this allows for safe generics without any explicit annotations

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

                    Extra Notes:
                    Optimize away method chaining, as this is the same as parsing out token from Box<Expr::Const(token, _)> and calling self.get_type(token)
                    If this resolves to a valid Type::Func(..), then extract the tuple's value.
                */
                let (number_of_parameters, return_type, function_stmt) =
                    match self.resolve_expression(callee_identifier_expr)? {
                        Type::Func(number_of_parameters, return_type, function_stmt) => {
                            (number_of_parameters, return_type, function_stmt)
                        }

                        value_type => {
                            // @todo fix error and show the actual value type used
                            return Err(TypeCheckerError::InternalError(
                                "TESTING - cannot call 'value_type' as a function",
                            ));
                        }
                    };

                // @todo Add additional check if supporting variadic functions
                // Ensure that the number of arguments matches the number of parameters defined
                if arguments.len() != number_of_parameters {
                    return Err(TypeCheckerError::InternalError(
                        "TESTING - Call - different numbers of arguments",
                    ));
                }

                // Create a fixed length vec of arg types and get the arg types by resolving the args individually
                let mut argument_types: Vec<Type> = Vec::with_capacity(arguments.len());
                for ref arg in arguments {
                    argument_types.push(self.resolve_expression(arg)?);
                }

                // Type check the function again, this time with the types of the arguments as types of the parameters
                if let Stmt::Func(ref identifier_token, ref params, ref body) = *function_stmt {
                    // The type of the call expression is the return type of the function called after resolving it
                    self.check_function(identifier_token, params, argument_types, body)?
                } else {
                    panic!("Internal Error: Expected Stmt::Func to be stored in Type::Func")
                }
            }
            Expr::Grouping(ref expr) => self.resolve_expression(expr)?,
            Expr::Literal(ref literal) => match literal {
                // @todo Might need to split into signed and unsigned num
                Literal::Number(_) => Type::Number,
                Literal::String(_) => Type::String,
                Literal::Bool(_) => Type::Bool,
                Literal::Null => Type::Null,
            },
            Expr::Array(_, ref elements) => {
                let array_element_type = self.resolve_expression(&elements[0])?;

                // Resolve for elements[1..] of the array, where all elements are expressions
                for element in elements.into_iter().skip(1) {
                    if self.resolve_expression(element)? != array_element_type {
                        return Err(TypeCheckerError::InternalError("TESTING - Array"));
                    }
                }

                Type::Array(Box::new(array_element_type))
            }
            Expr::ArrayAccess(ref array_identifier_expr, ref index_expression) => {
                // @todo Ensure that the indexing expression is a unsigned integer, not just a number, to remove the runtime check
                if self.resolve_expression(index_expression)? != Type::Number {
                    return Err(TypeCheckerError::InternalError("TESTING"));
                }

                // This is the same as parsing out token from, Box<Expr::Const(token, _)> and calling self.get_type(token)
                // If this resolves to a valid Type::Array(..) type, then extract the 'array_element_type'
                match self.resolve_expression(array_identifier_expr)? {
                    Type::Array(array_element_type) => *array_element_type,

                    value_type => {
                        // @todo fix error and show the actual value type used
                        return Err(TypeCheckerError::InternalError(
                            "TESTING - cannot access 'value_type' as an array",
                        ));
                    }
                }
            }
            Expr::Logical(ref left, _, ref right) => {
                let l_type = self.resolve_expression(left)?;
                let r_type = self.resolve_expression(right)?;

                // Check that the first type is Bool, and if so, check if second type is bool
                // Doing this because Rust does not support comparison operator chaining 'l_type == r_type == Type::Bool'
                if (l_type == Type::Bool) && (r_type == Type::Bool) {
                    // Logical expressions always evaluate to a value of Boolean type
                    Type::Bool
                } else {
                    return Err(TypeCheckerError::InternalError(
                        "TESTING - Logical expressions must be bool",
                    ));
                }
            }
            Expr::Unary(ref operator, ref expr) => {
                let expr_type = self.resolve_expression(expr)?;

                match &operator.token_type {
                    TokenType::Bang => {
                        if expr_type == Type::Bool {
                            Type::Bool
                        } else {
                            return Err(TypeCheckerError::InternalError(
                                "TESTING - Unary NOT expressions must be bool",
                            ));
                        }
                    }
                    TokenType::Minus => {
                        if expr_type == Type::Number {
                            Type::Number
                        } else {
                            return Err(TypeCheckerError::InternalError(
                                "TESTING - Unary NEGATE expressions must be Number",
                            ));
                        }
                    }
                    // Alternative syntax
                    // TokenType::Minus if expr_type == Type::Number => Type::Number,
                    // TokenType::Minus => {
                    //     return Err(TypeCheckerError::InternalError(
                    //         "TESTING - Logical expressions must be bool",
                    //     ));
                    // }
                    invalid_token_type => panic!(
                        "Internal Error: Found {:?} in Expr::Unary",
                        invalid_token_type
                    ),
                }
            }

            #[allow(unreachable_patterns)]
            ref unmatched_expr_variant => panic!("{}", unmatched_expr_variant),
        })
    }

    fn resolve_function(
        &mut self,
        function_name: String,
        param_tokens: &Vec<Token>,
        body: &Stmt,
    ) -> Result<Type, TypeCheckerError> {
        // Save parent function's name first if any before assigning the name of the current function
        let parent_function_name = self.current_function.clone();
        self.current_function = Some(function_name);

        self.begin_scope();

        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();
        for param_token in param_tokens {
            // Save type of every parameter into scope as Type::Lazy during this function definition type checking process,
            // To defer type checking for statements that use these parameters till function call type checks,
            // And during which the type of the arguments will be available
            scope.insert(param_token.lexeme.as_ref().unwrap().clone(), Type::Lazy);
        }

        // Assuming most functions only have 1 return statement
        let mut return_types = Vec::<Type>::with_capacity(1);

        // Body must be a block statement, even for anonymous arrow functions
        // arrow functions is just syntatic sugar in this implementation, so they are actually also parsed into block statements
        if let &Stmt::Block(ref stmts) = body {
            for stmt in stmts {
                let stmt_type = self.resolve_statement(stmt)?;
                if let Type::Return(_) = stmt_type {
                    return_types.push(stmt_type);
                }
            }
        } else {
            panic!("Internal Error: Function body can only be Stmt::Block");
        };

        self.end_scope();

        // Restore the name of the parent function
        self.current_function = parent_function_name;

        Ok(
            // If there are no return statements, default return type is Null
            if return_types.is_empty() {
                Type::Null
            } else {
                // @todo Optimize by skipping the first element, otherwise it will be compared with itself
                for return_type in &return_types {
                    if return_type != &return_types[0] {
                        return Err(TypeCheckerError::InternalError(
                            "TESTING - Function must have the same return type throughout the function body"
                        ));
                    }
                }

                // If all return types are the same, then use first type as function return type
                return_types[0].clone()
            },
        )
    }

    fn check_function(
        &mut self,
        identifier_token: &Token,
        param_tokens: &Vec<Token>,
        argument_types: Vec<Type>,
        body: &Stmt,
    ) -> Result<Type, TypeCheckerError> {
        // Check if the function is a recursive one, by checking if the name of the function called is the same as the parent function
        if let Some(ref parent_function_name) = self.current_function {
            if parent_function_name == identifier_token.lexeme.as_ref().unwrap() {
                return Ok(Type::Lazy);
            }
        }
        // Save parent function's name first if any before assigning the name of the current function
        let parent_function_name = self.current_function.clone();
        self.current_function = Some(identifier_token.lexeme.as_ref().unwrap().clone());

        self.begin_scope();

        // A scope is always expected to exists, including the global top level scope
        let scope = self.scopes.last_mut().unwrap();
        for (i, param_token) in param_tokens.into_iter().enumerate() {
            scope.insert(
                param_token.lexeme.as_ref().unwrap().clone(),
                argument_types[i].clone(),
            );
        }

        // Assuming most functions only have 1 return statement
        let mut return_types = Vec::<Type>::with_capacity(1);

        // Body must be a block statement, even for anonymous arrow functions
        // arrow functions is just syntatic sugar and are also parsed into block statements
        if let &Stmt::Block(ref stmts) = body {
            for stmt in stmts {
                let stmt_type = self.resolve_statement(stmt)?;
                if let Type::Return(_) = stmt_type {
                    return_types.push(stmt_type);
                }
            }
        } else {
            panic!("Internal Error: Function body can only be Stmt::Block");
        };

        self.end_scope();

        // Restore the name of the parent function
        self.current_function = parent_function_name;

        Ok(
            // If there are no return statements, default return type is Null
            if return_types.is_empty() {
                Type::Null
            } else {
                // @todo Optimize by skipping the first element, otherwise it will be compared with itself
                for return_type in &return_types {
                    if return_type != &return_types[0] {
                        return Err(TypeCheckerError::InternalError(
                            "TESTING - Function must have the same return type throughout the function body"
                        ));
                    }
                }

                // If all return types are the same, then use first type as function return type
                return_types[0].clone()
            },
        )
    }
}
