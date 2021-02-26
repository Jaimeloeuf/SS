use super::error::RuntimeError;
use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::environment::Environment;
use crate::literal::Literal;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token_type::TokenType;
use crate::value::value::Value;

pub struct Interpreter {
    // Env tracks the current environment, changing as the interpreter enter and exit local scopes
    env: Rc<RefCell<Environment>>,
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
            match interpreter.interpret_stmt(stmt) {
                // This value is technically only meaningful when using the repl/toplevel
                // Ok(value) => println!("Evaluated to {:?}", value),
                Ok(value) => {}

                // Interpreter stop interpreting code once there is any runtime error
                Err(err) => return Some(err),
            }
        }

        None
    }

    // Returns a Value Option as not every statement evaluates to a Value
    fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, RuntimeError> {
        // Wrap match expression in Ok variant instead of wrapping Value options with Ok variant in every arm
        // Err option inside match expression cannot evaluate and return implicitly due to the Ok wrapping,
        // thus it needs to be explicitly returned to break out of this Ok variant wrapping.
        Ok(match stmt {
            // @todo Perhaps simplify this by wrapping the whole interpret_expr in a Option<Value> too to dont have to unwrap and rewrap here
            Stmt::Expr(ref expr) => Some(self.interpret_expr(expr)?),

            // Block statement, groups statements together in the same scope for execution
            Stmt::Block(ref statements) => {
                let parent_env = Rc::clone(&self.env);

                // Create new environment/scope for current block with existing environment/scope as the parent/enclosing environment
                let current_env = Environment::new(Some(Rc::clone(&self.env)));
                // Set the new environment directly onto the struct, so other methods can access it directly
                // @todo Can be better written, by changing all the methods to take current scope as function arguement,
                // @todo instead of saving current environment temporarily and attaching the new environment to self.
                self.env = Rc::new(RefCell::new(current_env));

                let mut return_value = None;
                for ref stmt in statements {
                    // @todo Deal with the errors better to drop the memory of env. Will this be done automatically?
                    return_value = self.interpret_stmt(stmt)?;

                    // Break out of execution once return statement is executed
                    if let Stmt::Return(ref _token, ref _expr) = stmt {
                        break;
                    }

                    // @todo If we allowed implicit returns
                    // if "stmt is last stmt of block" && return_value.is_some() {
                    //     break;
                    // }
                }

                // Reset parent environment back onto the struct once block completes execution
                // The newly created current environment for this block will be dropped once function exits
                self.env = parent_env;

                // Return the return value of the block if there is any
                return_value
            }

            // @todo Does return stmt really need to store the token?
            // Return statment is just like Stmt::Expr where it just returns the evaluated expression
            Stmt::Return(_, ref expr) => Some(self.interpret_expr(expr)?),

            // @todo Maybe do a pre-check in parser somehow to ensure that the evaluated Value must be a Bool
            Stmt::If(ref condition, ref true_branch, ref else_branch) => {
                // If/Else version using bool_or_err method on Value
                self.interpret_stmt(if self.interpret_expr(condition)?.bool_or_err(
                    "Invalid condition value type, only Boolean values can be used as conditionals!"
                )? {
                    true_branch
                } else {
                    // Only return else_branch if any, else end function
                    match else_branch {
                        Some(ref else_branch) => else_branch,
                        _ => return Ok(None), // Return to break out of this expr passed into interpret_stmt method call
                    }
                })?
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
                // Although the token definitely have a literal string variant if parsed correctly,
                // Rust treats this as a pattern matching context with a refutable pattern, so we have to deal with the else case,
                // Which only happens if parser failed to save String literal for Identifier type Token
                // Reference: https://stackoverflow.com/questions/41573764
                if let Literal::String(ref identifier) = token.literal.as_ref().unwrap() {
                    /*
                        self.env
                            .borrow_mut()
                            .define(identifier.to_string(), self.interpret_expr(expr)?);

                        Cannot compress code like above, because when running call to define method,
                        interpret_expr is ran first, and if the expr is a Expr::Const or something that accesses env,
                        with a borrow() or borrow_mut() then there will be a borrow error and panic.
                        Although it isn't very intuitive, since we expect self.interpret_expr(expr)? to run to completion
                        before control is handed over to the define method call, it will not work if chained.

                        // The above code will cause SS code on the Next line to fail with a
                        // thread 'main' panicked at 'already mutably borrowed: BorrowError', src\interpreter\interpreter.rs
                        const a = 1;
                        const b = a + 2; // Fails when we try to access a value from env to assign to a new key in env
                    */
                    let expression = self.interpret_expr(expr)?;
                    self.env
                        .borrow_mut()
                        .define(identifier.to_string(), expression);
                    None
                } else {
                    // If somehow a identifier token does not have a string literal, then token Display trait is not helpful for debugging,
                    // Because it attempts to print out the string literal which we know is missing, thus print with debug symbol instead
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
                    // @todo Using debug symbol to print as stmt does not implement Display trait yet
                    "Failed to interpret statement.\nUnimplemented Stmt variant: {:?}",
                    unmatched_stmt_variant
                )));
            }
        })
    }

    fn interpret_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            // Using *Literal, to get the value from within the variant
            Expr::Literal(literal) => match *literal {
                Literal::Number(number) => Ok(Value::Number(number)),
                // Use a ref here to prevent moving it, and clone the string
                Literal::String(ref string) => Ok(Value::String(string.clone())),
                Literal::Bool(bool) => Ok(Value::Bool(bool)),
                Literal::Null => Ok(Value::Null),
            },

            // A Const expression evaluates to the value stored in the environment identified by the Const's identifier
            // Distance is not implemented for now
            Expr::Const(ref token, ref _distance) => {
                // Although the token definitely have a literal string variant if parsed correctly,
                // Rust treats this as a pattern matching context with a refutable pattern, so we have to deal with the else case,
                // Which only happens if parser failed to save String literal for Identifier type Token
                // Reference: https://stackoverflow.com/questions/41573764
                if let Literal::String(ref identifier) = token.literal.as_ref().unwrap() {
                    // @todo
                    // Reference: https://stackoverflow.com/questions/30414424
                    // Should use get_ref here instead of get to avoid cloning the value
                    // But that would require changing the method's return type
                    // Should we even move out a Value in the first place? Shouldnt all the values be immutable?
                    // Or perhaps return a mutable ref from env hashmap and every modification is made directly on the hashmap without needing additional update logic?
                    match self.env.borrow().get(identifier) {
                        Some(value) => Ok(value),
                        // @todo When not found, should it be an environment error or runtime error?
                        // Technically should be Runtime error, because it is caused by the user using a invalid identifier
                        // Environment errors are reserved for when there is a valid identifier but not found in environment
                        None => Err(RuntimeError::UndefinedIdentifier(identifier.clone())),
                    }
                } else {
                    // Unlikely to happen because this will probably be caught by interpret_stmt's Const logic when setting a value
                    //
                    // If somehow a identifier token does not have a string literal, then token Display trait is not helpful for debugging,
                    // Because it attempts to print out the string literal which we know is missing, thus print with debug symbol instead
                    Err(RuntimeError::InternalError(format!(
                        "Runtime Error: Unable to read value on const identifier -> {:?}\n{}",
                        token, "Parsing error: Const identifier missing string literal\n"
                    )))
                }
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

                    operator => Err(RuntimeError::InternalError(format!(
                        "Invalid unary operator: {:?}",
                        operator
                    ))),
                }
            }

            // Binary expression with an operator and 2 operands
            Expr::Binary(ref left, ref operator, ref right) => {
                // This evaluates the Binary expression from left to right
                // In certain cases, we might want to change this, to support bool short circuiting
                let left_value = self.interpret_expr(left)?;
                let right_value = self.interpret_expr(right)?;

                match &operator.token_type {
                    TokenType::Plus => {
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
                            _ => Err(RuntimeError::TypeError(
                                // @todo Show types used
                                // "Invalid types used for addition!",
                                "Invalid types used for addition!".to_string(),
                            )),
                        }
                    }

                    TokenType::Minus => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Number(left_number - right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for subtraction!",
                                "Invalid types used for subtraction!".to_string(),
                            )),
                        }
                    }

                    TokenType::Star => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Number(left_number * right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for multiplication!",
                                "Invalid types used for multiplication!".to_string(),
                            )),
                        }
                    }

                    TokenType::Slash => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Number(left_number / right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for division!",
                                "Invalid types used for division!".to_string(),
                            )),
                        }
                    }

                    // @todo Can we add a try/catch? Then if fail, we return the Err(InternalErro or TypeErroor for cannot compare)
                    // @todo Allows for comparison of primitive types and string so far, but might want to test for complex types like Functions
                    TokenType::EqualEqual => {
                        // Can do direct comparison here as long as Value enum derives the PartialEq trait
                        Ok(Value::Bool(left_value == right_value))
                    }
                    TokenType::BangEqual => {
                        // Can do direct comparison here as long as Value enum derives the PartialEq trait
                        Ok(Value::Bool(left_value != right_value))
                    }

                    TokenType::Greater => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Bool(left_number > right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for comparison!",
                                "Invalid types used for comparison!".to_string(),
                            )),
                        }
                    }

                    TokenType::GreaterEqual => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Bool(left_number >= right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for comparison!",
                                "Invalid types used for comparison!".to_string(),
                            )),
                        }
                    }

                    TokenType::Less => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Bool(left_number < right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for comparison!",
                                "Invalid types used for comparison!".to_string(),
                            )),
                        }
                    }

                    TokenType::LessEqual => {
                        match (left_value, right_value) {
                            (Value::Number(left_number), Value::Number(right_number)) => {
                                Ok(Value::Bool(left_number <= right_number))
                            }
                            _ => Err(RuntimeError::TypeError(
                                // "Invalid types used for comparison!",
                                "Invalid types used for comparison!".to_string(),
                            )),
                        }
                    }
                    _ => Err(RuntimeError::InternalError(format!(
                        "Invalid binary operator: {}",
                        operator
                    ))),
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

            unmatched => Err(RuntimeError::InternalError(format!(
                "Unimplemented expr type -> {}",
                unmatched
            ))),
        }
    }
}
