use super::error::RuntimeError;
use crate::literal::Literal;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::token_type::TokenType;
use crate::value::value::Value;

pub struct Interpreter {
}

impl Interpreter {
    pub fn interpret(stmts: Vec<Stmt>) -> Option<RuntimeError> {
        let mut interpreter = Interpreter {
        };

        // Loop through all Expr/Stmt to evaluate and run them, returning any errors
        for stmt in stmts.iter() {
            match interpreter.interpret_stmt(stmt) {
                Ok(value) => println!("Evaluated to {:?}", value),
                Err(err) => {
                    // @todo Use this without the debug symbol using Display trait
                    // @todo Delete this println, and let method caller handle the error
                    // println!("Error! {}", err);
                    println!("Error! {:?}", err);
                    return Some(err);
                }
            }
        }

        None
    }

    fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
        Err(RuntimeError::InternalError(
            "Failed to interpret statement".to_string(),
        ))
    }

    fn interpret_expr(&mut self, stmt: &Expr) -> Result<Value, RuntimeError> {
        match stmt {
            // Using *Literal, to get the value from within the variant
            Expr::Literal(literal) => match *literal {
                Literal::Number(number) => Ok(Value::Number(number)),
                // Use a ref here to prevent moving it, and clone the string
                Literal::String(ref string) => Ok(Value::String(string.clone())),
                Literal::Bool(bool) => Ok(Value::Bool(bool)),
                Literal::Null => Ok(Value::Null),
            },

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

            unmatched => Err(RuntimeError::InternalError(format!(
                "Unimplemented expr type -> {}",
                unmatched
            ))),
        }
    }
}
