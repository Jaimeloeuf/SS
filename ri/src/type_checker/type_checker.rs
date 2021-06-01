use super::error::TypeCheckerError;
use super::TypeChecker;

use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token::Token;

// Might need to use macro for the diff types...?
fn is_equal() -> bool {
    false
}

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
    fn resolve_ast(&mut self, ast: &Vec<Stmt>) -> Result<(), TypeCheckerError> {
        for ref stmt in ast {
            self.resolve_statement(stmt)?;
        }

        Ok(())
    }

    // @todo Use reference to the string instead of having to own it for lexeme.clone()
    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<(), TypeCheckerError> {
        Ok(match *stmt {
            Stmt::Expr(ref expr) => self.resolve_expression(expr)?,
            Stmt::Block(ref stmts) => {
                self.begin_scope();
                self.resolve_ast(stmts)?;
                self.end_scope();
            }
            Stmt::Const(ref token, ref expr) => {
                self.declare(token)?;
                self.resolve_expression(expr)?;
                self.define(token);
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
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(ref else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Stmt::Print(ref expr) => self.resolve_expression(expr)?,
            Stmt::Return(ref token, ref expr) => {
                // If not in any function, return statements are not allowed
                if !self.in_function {
                    return Err(TypeCheckerError::ReturnOutsideFunction(token.clone()));
                }

                self.resolve_expression(expr)?;
            }
            Stmt::While(ref condition, ref body) => {
                self.resolve_expression(condition)?;
                self.resolve_statement(body)?;
            }

            #[allow(unreachable_patterns)]
            ref unmatched_stmt_variant => panic!("{}", unmatched_stmt_variant),
        })
    }

    fn resolve_expression(&mut self, expr: &Expr) -> Result<(), TypeCheckerError> {
        match *expr {
            Expr::Const(ref token, ref distance_value_in_ast_node) => {
                let distance = self.resolve_identifier_distance(token)?;

                // @todo UNSAFE WAY used temporarily for testing! See alternatives below
                unsafe {
                    let mutable_pointer = distance_value_in_ast_node as *const usize as *mut usize;
                    *mutable_pointer = distance;
                }
                // Alternative 1 is to call resolve_expression with mut reference to the expression
                // *distance_value_in_ast_node = self.resolve_identifier_distance(token.lexeme.as_ref().unwrap().clone());

                // Alternative 2 is to save distance value into a side table instead of saving directly into the AST node
                // Problem with this is we cannot have identifiers of the same name, even in different scopes if using identifier string as key
                // Perhaps use the string and line number? But this will prevent minification....
                // let identifier = token.lexeme.as_ref().unwrap();
                // side_table.insert(identifier.clone(), self.resolve_identifier_distance(identifier.clone()));

                // println!("t {:?} -> {}", token.lexeme, distance_value_in_ast_node);
            }
            Expr::AnonymousFunc(ref stmt) => {
                // Expr::AnonymousFunc is a wrapper for Stmt::AnonymousFunc, thus use resolve_statement to handle Stmt::AnonymousFunc
                self.resolve_statement(stmt)?;
            }
            Expr::Binary(ref left, _, ref right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Call(ref callee, ref arguments, _) => {
                self.resolve_expression(callee)?;

                for ref arg in arguments {
                    self.resolve_expression(arg)?;
                }
            }
            Expr::Grouping(ref expr) => {
                self.resolve_expression(expr)?;
            }
            Expr::Literal(_) => {}
            Expr::Array(_, ref elements) => {
                // Resolve for every single element in the array, where all elements are expressions
                for element in elements {
                    self.resolve_expression(element)?;
                }
            }
            Expr::ArrayAccess(ref array, ref index_expression) => {
                self.resolve_expression(array)?;
                self.resolve_expression(index_expression)?;
            }
            Expr::Logical(ref left, _, ref right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Unary(_, ref expr) => {
                self.resolve_expression(expr)?;
            }
            #[allow(unreachable_patterns)]
            ref unmatched_expr_variant => panic!("{}", unmatched_expr_variant),
        };

        Ok(())
    }

    // Returns the Number of scope to traverse up to find the identifier's definition
    // E.g. 0 means defined in the same scope and 2, means defined 2 scopes above current scope.
    //
    // should it be definition instead? So prevent const a = a;  (not definition)
    // This will go up through all the scopes looking for the identifier's "declaration"
    // If the definition is still not found after reaching the global scope, return an Undefined Identifier error
    fn resolve_identifier_distance(&self, token: &Token) -> Result<usize, TypeCheckerError> {
        let identifier = token.lexeme.as_ref().unwrap().clone();

        // Simple optimization, as identifiers are usually defined in the same scope more often than not
        // Ok to unwrap, as 'scopes' vector will never be empty in this method as global scope only deleted in type_checker::resolve()
        if self.scopes.last().unwrap().contains_key(&identifier) {
            return Ok(0);
        }

        // Convert scopes vector into Iter type and reverse it to traverse up from local scope all the way to top level global scope
        // Skip the first scope, which is the local scope since we already check the local scope in the if statement above.
        // Then enumerate it to get both the scope and the index (which is the number of scopes from current local scope)
        for (i, ref scope) in self.scopes.iter().rev().skip(1).enumerate() {
            if scope.contains_key(&identifier) {
                // println!("foound! {} -> {}", identifier, i + 1);
                // Return 'i + 1', instead of 'i', because we skipped the first one, but i still starts from 0
                // Where scope distance of 0, means current local scope.
                return Ok(i + 1);
            }
            // println!("NO FIND! {} -> {}", identifier, i + 1);
        }

        // If identifier is not found in all scopes (even in global scope) then it is undefined.
        Err(TypeCheckerError::UndefinedIdentifier(token.clone()))
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

            _ => {
                return Err(TypeCheckerError::InternalError(
                    "Function body can only be Stmt::Block",
                ))
            }
        }

        self.end_scope();
        self.in_function = is_parent_in_function;
        Ok(())
    }
}
