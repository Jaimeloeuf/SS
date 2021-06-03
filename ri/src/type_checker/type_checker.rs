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
            in_function: false,

            // @todo A better way other than hardcoding all identifiers in
            globals: vec!["clock"],
        };

        // Create first new scope for the global scope and insert in identifiers
        type_checker.begin_scope();

        // @todo Make it better then.. Cloning it because cannot have ref and mut ref to type_checker at the same time....
        type_checker.define_globals(type_checker.globals.clone());

        type_checker.resolve_ast(ast)?;
        type_checker.end_scope();

        Ok(())
    }

    // Resolve statements 1 by 1
    // Change name to resolve node? Cause we start from a single node and then can be called recursively for every node
    fn resolve_ast(&mut self, ast: &Vec<Stmt>) -> Result<Type, TypeCheckerError> {
        for ref stmt in ast {
            self.resolve_statement(stmt)?;
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
                return self.resolve_expression(expr);
            }
            Stmt::Block(ref stmts) => {
                self.begin_scope();
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
            Stmt::Func(ref token, ref params, ref body) => {
                // Declare and define to allow function to refer to itself recursively
                self.declare_and_define(token)?;
                self.resolve_function(params, body)?;
            }
            Stmt::AnonymousFunc(ref params, ref body) => {
                // Unlike Stmt::Func, dont need to declare and define since Anonymous Functions are nameless, and will be bound to a Const identifier
                self.resolve_function(params, body)?;
            }
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

            // @todo Add new arithmetic expr
            // Binary expressions holds both equality/inequality checks, and arithmetic operations
            Expr::Binary(ref left, _, ref right) => {
                let l_type = self.resolve_expression(left)?;
                let r_type = self.resolve_expression(right)?;

                // Plus,
                // Slash,
                // Star,
                // if operator.token_type == TokenType::Minus {
                //     return Type::Number
                // }

                if l_type == r_type {
                    // Need to return here too?
                    l_type
                } else {
                    return Err(TypeCheckerError::InternalError("TESTING - Binary"));
                }
            }
            Expr::Call(ref callee, ref arguments, _) => {
                if self.resolve_expression(callee)? != Type::Func(_, _) {
                    //
                }

                // Create a fixed length vec of arg types and get the arg types by resolving the args individually
                let mut argument_types: Vec<Type> = Vec::with_capacity(arguments.len());
                for ref arg in arguments {
                    argument_types.push(self.resolve_expression(arg)?);
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

                // Resolve for every single element in the array, where all elements are expressions
                for element in elements {
                    if self.resolve_expression(element)? != array_element_type {
                        return Err(TypeCheckerError::InternalError("TESTING - Array"));
                    }
                }

                Type::Array(Box::new(array_element_type))
            }
            Expr::ArrayAccess(ref array_identifier_expr, ref index_expression) => {
                // @todo Ensure that the indexing expression is a unsigned integer, not just a number
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
        param_tokens: &Vec<Token>,
        body: &Stmt,
    ) -> Result<(), TypeCheckerError> {
        // Save parent status first before assigning in_function as true
        let is_parent_in_function = self.in_function;
        self.in_function = true;

        self.begin_scope();

        // Declare and define every token
        for token in param_tokens {
            self.declare_and_define(token)?;
        }

        // Body must be a block statement, even for anonymous arrow functions
        // arrow functions is just syntatic sugar in this implementation, so they are actually also parsed into block statements
        match body {
            &Stmt::Block(ref stmts) => {
                for stmt in stmts {
                    self.resolve_statement(stmt)?;
                }
            }

            _ => panic!("Internal Error: Function body can only be Stmt::Block"),
        }

        self.end_scope();
        self.in_function = is_parent_in_function;
        Ok(())
    }
}
