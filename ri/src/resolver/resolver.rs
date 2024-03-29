use super::error::ResolvingError;

use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token::Token;

use std::collections::hash_map::HashMap;

// Add lifetime specifier to String so that we can use ref of string instead of constantly cloning strings
pub struct Resolver {
    // Using a vec as a "Stack" data structure
    // @todo Might change this to a LinkedList
    // Pub to make this accessible by utility module
    pub scopes: Vec<HashMap<String, bool>>,

    // Tracker to see if currently in a function or not
    // Used to see if return statements are valid
    in_function: bool,

    // Field holding a vector of global identifiers
    // Used by declare utility method to check if the identifier is a global identifier to give users a more specific error message
    pub globals: Vec<&'static str>,
}

impl Resolver {
    // Associated function to resolve a AST
    pub fn resolve(ast: &Vec<Stmt>) -> Result<(), ResolvingError> {
        // Create resolver instance internally
        let mut resolver = Resolver {
            scopes: Vec::new(),
            in_function: false,

            // @todo A better way other than hardcoding all identifiers in
            globals: vec!["clock"],
        };

        // Create first new scope for the global scope and insert in identifiers
        resolver.begin_scope();

        // @todo Make it better then.. Cloning it because cannot have ref and mut ref to resolver at the same time....
        resolver.define_globals(resolver.globals.clone());

        for stmt in ast {
            // Since return statements are illegal outside of a function body, statements in global scope can never be halting.
            // @todo Store all the errors and add synchronization point so that multiple errors can be found at once
            resolver.resolve_statement(stmt)?;
        }

        resolver.end_scope();

        Ok(())
    }

    /// Resolve statements in a block statement 1 by 1
    ///
    /// Since statements can be halting, this method checks for unreachable statements if a statement is halting,
    /// regardless if function or none function block, make sure only the last stmt of this block is halting.
    /// Errors on unreachable code, else bubbles up the halting status of the last statement.
    fn resolve_block_statement(&mut self, block_statement: &Stmt) -> Result<bool, ResolvingError> {
        if let &Stmt::Block(ref stmts, optional_line_number) = block_statement {
            // Error on empty block statement, checked here instead of parser to avoid false positives. See parser for details.
            if stmts.is_empty() {
                Err(ResolvingError::EmptyBlockStatement(
                    // Unwrap to get line number directly, because only block statements of anonymous functions do not have line numbers,
                    // And they will not be empty, since the block statement is a desugared syntax guaranteed to be none empty.
                    optional_line_number.unwrap(),
                ))

                // Alternatively if empty blocks are accepted, then this block is not halting as there is no return.
                // Ok(false)
            } else {
                // Loop through all the statements in block statement till the second last one,
                // The last statement will be resolved and checked seperately as it does not need an unreachable code check
                for stmt in stmts[..stmts.len() - 1].iter() {
                    let halting = self.resolve_statement(stmt)?;

                    // If statement is halting, there is unreachable code in this block statement since this cannot be the last statement
                    if halting {
                        // Create the appropriate unreachable code message based on the current statement
                        // Only these stmt types (Return / Block / If / While) can be halting
                        // Create error message here instead the display trait implementation, as it is memory intensive to clone stmt
                        let (line_number, msg) = match stmt {
                            Stmt::Return(_, line_number) => (line_number, "'return' statement"),
                            Stmt::Block(_, Some(line_number)) => (line_number, "'block' statement"),
                            Stmt::If(_, _, _, line_number) => (line_number, "'if-else' statement"),
                            Stmt::While(_, _, line_number) => (line_number, "'while' loop"),

                            // All other statement types cannot be halting, thus they will not appear here
                            _ => panic!("Invalid 'unreachable' statement: {:#?}", stmt),
                        };

                        // Return error and stop resolving this path
                        return Err(ResolvingError::UnreachableCode(format!(
                            "[line {}] Unreachable code found after this {}",
                            line_number, msg
                        )));
                    }
                }

                // Get the last statement and unwrap it directly (it is garunteed to not be empty after parsing),
                // Resolve the statement and return it's halting status as the halting status of the block statement.
                self.resolve_statement(stmts.last().unwrap())
            }
        } else {
            panic!(
                "resolve_block_statement method only accepts Stmt::Block, found: {:#?}",
                block_statement
            );
        }
    }

    // @todo Use reference to the string instead of having to own it for lexeme.clone()
    /// This method returns a bool indicating if the statement is halting and is used to determine if there is unreachable code.
    ///
    /// Halting, refers to whether any other statements can still be executed after this statement.
    /// Halting statements contain return statements either directly or nested within, and all statements after return is unreachable.
    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<bool, ResolvingError> {
        match *stmt {
            // No expression is halting, so by extension, the expression stmt is not halting
            // Why are unused values caught in type checker instead of resolver?
            // Because although we dont allow Expression statements like, `1 + 1`
            // Expression statements like `call_function()` should be allowed
            // And since resolver have no way of telling if `1 + 1` produces a value or if call_function() does,
            // Type checker is used as it can already get the type back, which can be used to make ignore a part of the type system
            Stmt::Expr(ref expr) => self.resolve_expression(expr)?,
            // A block stmt can contain nested return statements, therefore a block stmt can be halting
            Stmt::Block(_, _) => {
                self.begin_scope();
                // Pass whole statement in to resolve instead of just the inner vec of statements
                // The returned value does not need to be unwrapped since this nested halting status is bubbled up immediately
                let nested_halting_status = self.resolve_block_statement(stmt);
                self.end_scope();

                // Bubble up the halting status of the block statement
                return nested_halting_status;
            }

            // Const definitions are not halting, even when used to bind an anonymous function.
            // Because nested return(s) within anonymous functions does not halt the code within the const binding's scope.
            // i.e. a const definition is not halting at its scope depth as it is unaffected by nested halting code.
            Stmt::Const(ref token, ref expr) => {
                self.declare(token)?;
                self.resolve_expression(expr)?;
                self.define(token);
            }

            // Functions are self contained, so they are not halting, even if there is a return statement within it.
            // That return statement means that it is halting at that point in the inner function body, not the outer block.
            Stmt::Func(ref token, ref params, ref body) => {
                // Declare and define to allow function to refer to itself recursively
                self.declare_and_define(token)?;
                self.resolve_function(params, body)?;
            }
            Stmt::AnonymousFunc(ref params, ref body) => {
                // Unlike Stmt::Func, dont need to declare and define since Anonymous Functions are nameless, and will be bound to a Const identifier
                self.resolve_function(params, body)?;
            }

            // A if statement is only halting, if both the if and else blocks are halting.
            //
            // Because by definition, a standalone if statement (no else branch) may or may not execute its body,
            // so even if the if block is halting, it does not garuntee that the statement itself is halting,
            // since the condition may be evaluated to false.
            // However if both if and else branches are defined, it means that the execution path MUST go down either of the branches.
            // In that case the statement as a whole is halting, if both the if and else branches are halting.
            Stmt::If(ref condition, ref then_branch, ref else_branch, _) => {
                self.resolve_expression(condition)?;
                // Unwrap to get halting status of branch body for comparison
                let then_branch_is_halting = self.resolve_statement(then_branch)?;
                if let Some(ref else_branch) = else_branch {
                    // Unwrap to get halting status of branch body for comparison
                    let else_branch_is_halting = self.resolve_statement(else_branch)?;
                    // If both branches are halting then this if stmt is halting, where True && True == True
                    return Ok(then_branch_is_halting && else_branch_is_halting);
                }
            }

            Stmt::Print(ref expr) => self.resolve_expression(expr)?,

            // Return statement is halting by definition
            Stmt::Return(ref expr, line_number) => {
                // If not in any function, return statements are not allowed
                if !self.in_function {
                    return Err(ResolvingError::ReturnOutsideFunction(line_number));
                }

                self.resolve_expression(expr)?;

                // A return statement is the only statement that is halting by definition.
                // All other statements, are only halting if it has a nested return statement somewhere.
                return Ok(true);
            }

            // Ignore statements are used to ignore evaluated values of expressions,
            // And since expressions are not halting by default, a ignore statement cannot be halting too
            // It is resolved the same as Stmt::Expr
            Stmt::Ignore(ref expr) => self.resolve_expression(expr)?,

            // While loops are halting if the loop body is halting. i.e. if there is a return statement within the loop body
            Stmt::While(ref condition, ref body, _) => {
                self.resolve_expression(condition)?;
                // The returned value does not need to be unwrapped since this nested halting status is bubbled up immediately
                return self.resolve_statement(body);
            }
        };

        // By default most statements are not halting
        Ok(false)
    }

    // All expressions are none halting, so there is no need for this method to return a halting indicator
    fn resolve_expression(&mut self, expr: &Expr) -> Result<(), ResolvingError> {
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
            Expr::Grouping(ref expr) => self.resolve_expression(expr)?,
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
        };

        Ok(())
    }

    // Returns the Number of scope to traverse up to find the identifier's definition
    // E.g. 0 means defined in the same scope and 2, means defined 2 scopes above current scope.
    //
    // This will go up through all the scopes looking for the identifier's "declaration"
    // If the definition is still not found after reaching the global scope, return an Undefined Identifier error
    fn resolve_identifier_distance(&self, token: &Token) -> Result<usize, ResolvingError> {
        let identifier = token.lexeme.as_ref().unwrap().clone();

        // Simple optimization, as identifiers are usually defined in the same scope more often than not
        // Ok to unwrap, as 'scopes' vector will never be empty in this method as global scope only deleted in resolver::resolve()
        if self.scopes.last().unwrap().contains_key(&identifier) {
            return Ok(0);
        }

        // Convert scopes vector into Iter type and reverse it to traverse up from local scope all the way to top level global scope
        // Skip the first scope, which is the local scope since we already check the local scope in the if statement above.
        // Then enumerate it to get both the scope and the index (which is the number of scopes from current local scope)
        for (i, ref scope) in self.scopes.iter().rev().skip(1).enumerate() {
            if scope.contains_key(&identifier) {
                // Set scope_depth to 'i + 1' instead of 'i', because the last scope is skipped,
                // But because 'i' is the enumerated index, it is unaffected by .skip(1) and starts at 0
                // Where scope distance of 0, means current local scope.
                return Ok(i + 1);
            }
        }

        // If identifier is not found in all scopes (even in global scope) then it is undefined.
        Err(ResolvingError::UndefinedIdentifier(token.clone()))
    }

    fn resolve_function(
        &mut self,
        param_tokens: &Vec<Token>,
        body: &Stmt,
    ) -> Result<(), ResolvingError> {
        // Save parent status first before assigning in_function as true
        let is_parent_in_function = self.in_function;
        self.in_function = true;

        self.begin_scope();

        // Declare and define every token
        for token in param_tokens {
            self.declare_and_define(token)?;
        }

        // Body must be a block statement, even for anonymous arrow functions
        // arrow functions is just syntatic sugar and are also parsed into block statements
        self.resolve_block_statement(body)?;

        self.end_scope();
        self.in_function = is_parent_in_function;
        Ok(())
    }
}
