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
}

impl Resolver {
    // Associated function to resolve a AST
    pub fn resolve(ast: &Vec<Stmt>) -> Result<(), ResolvingError> {
        // Create resolver instance internally
        let mut resolver = Resolver {
            scopes: Vec::new(),
            in_function: false,
        };

        // Create first new scope for the global scope and insert in identifiers
        resolver.begin_scope();
        // @todo A better way other than hardcoding all identifiers in
        resolver.define_globals(vec!["clock"]);

        resolver.resolve_ast(ast)?;
        resolver.end_scope();

        Ok(())
    }

    // Resolve statements 1 by 1
    // Change name to resolve node? Cause we start from a single node and then can be called recursively for every node
    fn resolve_ast(&mut self, ast: &Vec<Stmt>) -> Result<(), ResolvingError> {
        for ref stmt in ast {
            self.resolve_statement(stmt)?;
        }

        Ok(())
    }

    // @todo Use reference to the string instead of having to own it for lexeme.clone()
    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<(), ResolvingError> {
        Ok(match *stmt {
            Stmt::Expr(ref expr) => self.resolve_expression(expr),
            Stmt::Block(ref stmts) => {
                self.begin_scope();
                self.resolve_ast(stmts)?;
                self.end_scope();
            }
            Stmt::Const(ref token, ref expr) => {
                self.declare(token)?;
                self.resolve_expression(expr);
                self.define(token);
            }
            Stmt::Func(ref token, ref params, ref body) => {
                // Declare and define to allow function to refer to itself recursively
                self.declare_and_define(token)?;
                self.resolve_function(params, body)?;
            }
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                self.resolve_expression(condition);
                self.resolve_statement(then_branch)?;
                if let Some(ref else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Stmt::Print(ref expr) => self.resolve_expression(expr),
            Stmt::Return(ref token, ref expr) => {
                // If not in any function
                // Not in block
                if !self.in_function {
                    return Err(ResolvingError::ToplevelReturn(token.clone()));
                }

                self.resolve_expression(expr);
            }
            Stmt::While(ref condition, ref body) => {
                self.resolve_expression(condition);
                self.resolve_statement(body)?;
            }

            // @todo
            ref unmatched_stmt_variant => panic!("oops"),
        })
    }

    fn resolve_expression(&self, expr: &Expr) {
        match *expr {
            // @todo!!!
            Expr::Const(ref token, ref distance) => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(is_var_available) = scope.get(token.lexeme.as_ref().unwrap()) {
                        if !is_var_available {
                            // @todo Error
                        }
                    }
                }

                // Instead of saving onto the AST, what about we save into a side table?
                // *distance = self.resolve_local(token.lexeme.as_ref().unwrap().clone());
            }
            Expr::Binary(ref left, _, ref right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            Expr::Call(ref callee, ref arguments, _) => {
                self.resolve_expression(callee);

                for ref arg in arguments {
                    self.resolve_expression(arg);
                }
            }
            Expr::Grouping(ref expr) => {
                self.resolve_expression(expr);
            }
            Expr::Literal(_) => {}
            Expr::Logical(ref left, _, ref right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            Expr::Unary(_, ref expr) => {
                self.resolve_expression(expr);
            }
            Expr::Get(ref target, _) => {
                self.resolve_expression(target);
            }
            Expr::Set(ref target, _, ref value) => {
                self.resolve_expression(target);
                self.resolve_expression(value);
            }
            // Expr::Assign(ref token, ref  expr, ref  distance) => {
            //     self.resolve_expression(expr);
            //     *distance = self.resolve_local(token.lexeme.clone());
            // }

            // @todo
            ref unmatched_expr_variant => panic!("oops"),
        }
    }

    // fn resolve_local(&self, lexeme: String) -> Option<usize> {
    //     for (i, scope) in self.scopes.iter().rev().enumerate() {
    //         if scope.contains_key(&lexeme) {
    //             return Some(i);
    //         }
    //     }
    //     None
    // }

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

        match body {
            &Stmt::Block(ref stmts) => {
                for stmt in stmts {
                    self.resolve_statement(stmt)?;
                }
            }

            // @todo Really? What if we want to support single expression anonymous functions?
            _ => panic!("Function body can only be Stmt::Block"),
        }

        self.end_scope();
        self.in_function = is_parent_in_function;
        Ok(())
    }
}
