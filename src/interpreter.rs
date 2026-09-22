use std::collections::HashMap;

use crate::{
    ast,
    error::{Error, RuntimeError, RuntimeErrorKind::*},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
}

#[derive(Default, Debug)]
pub struct Interpreter {
    pub global_var: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            global_var: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[ast::Statement]) -> Result<(), Error> {
        for stmt in program {
            self.statement(stmt)?;
        }

        Ok(())
    }

    fn statement(&mut self, stmt: &ast::Statement) -> Result<(), Error> {
        match stmt {
            ast::Statement::Assignment(assign) => {
                let key = assign.iden.0.clone();
                let val = self.expression(&assign.expr)?;

                self.global_var.insert(key, val);
            }
            ast::Statement::Print(print) => {
                let val = self.expression(&print.0)?;

                match val {
                    Value::Number(num) => println!("{}", num),
                    Value::Boolean(bool) => println!("{}", bool),
                }
            }
            ast::Statement::IfStmt(if_else) => match self.expression(&if_else.if_cond)? {
                Value::Boolean(bool) => {
                    if bool {
                        for stmt in &if_else.if_stmt {
                            self.statement(stmt)?;
                        }
                    } else {
                        for stmt in &if_else.else_stmt {
                            self.statement(stmt)?;
                        }
                    }
                }
                _ => {
                    return Err(RuntimeError::new(
                        TypeMismatch,
                        format!("expected bool in if condition"),
                    )
                    .into());
                }
            },
        }

        Ok(())
    }

    fn expression(&self, expr: &ast::Expression) -> Result<Value, Error> {
        const ERROR: f64 = 1e-10;

        match expr {
            ast::Expression::Number(num) => Ok(Value::Number(num.0)),

            ast::Expression::Boolean(bool) => Ok(Value::Boolean(bool.0)),

            ast::Expression::Identifier(iden) => match self.global_var.get(&iden.0) {
                Some(val) => Ok(*val),
                None => Err(RuntimeError::new(
                    UndefinedVariable,
                    format!("undefined variable `{}`", iden.0),
                )
                .into()),
            },

            ast::Expression::UnaryOperation(unary) => {
                let value = self.expression(&unary.operand)?;

                match unary.operator {
                    ast::UnaryOperators::Add => {
                        if let Value::Number(_) = value {
                            Ok(value)
                        } else {
                            Err(RuntimeError::new(
                                TypeMismatch,
                                format!("expected number for unary `+`, found {:?}", value),
                            )
                            .into())
                        }
                    }
                    ast::UnaryOperators::Sub => {
                        if let Value::Number(num) = value {
                            Ok(Value::Number(-num))
                        } else {
                            Err(RuntimeError::new(
                                TypeMismatch,
                                format!("expected number for unary `-`, found {:?}", value),
                            )
                            .into())
                        }
                    }
                    ast::UnaryOperators::Not => {
                        if let Value::Boolean(bool) = value {
                            Ok(Value::Boolean(!bool))
                        } else {
                            Err(RuntimeError::new(
                                TypeMismatch,
                                format!("expected bool for unary `!`, found {:?}", value),
                            )
                            .into())
                        }
                    }
                }
            }

            ast::Expression::BinaryOperation(binary) => {
                let left = self.expression(&binary.left)?;
                let right = self.expression(&binary.right)?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Add => Ok(Value::Number(l + r)),
                        ast::BinaryOperators::Sub => Ok(Value::Number(l - r)),
                        ast::BinaryOperators::Mul => Ok(Value::Number(l * r)),
                        ast::BinaryOperators::Div => {
                            if r.abs() <= ERROR {
                                return Err(RuntimeError::new(
                                    DivisionByZero,
                                    format!("division by zero"),
                                )
                                .into());
                            }
                            Ok(Value::Number(l / r))
                        }
                        ast::BinaryOperators::Mod => Ok(Value::Number(l % r)),
                        ast::BinaryOperators::LThan => Ok(Value::Boolean(l < r)),
                        ast::BinaryOperators::GThan => Ok(Value::Boolean(l > r)),
                        ast::BinaryOperators::LThanEquals => Ok(Value::Boolean(l <= r)),
                        ast::BinaryOperators::GThanEquals => Ok(Value::Boolean(l >= r)),
                        ast::BinaryOperators::Equals => Ok(Value::Boolean((l - r).abs() < ERROR)),
                        ast::BinaryOperators::NotEquals => {
                            Ok(Value::Boolean((l - r).abs() > ERROR))
                        }
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("invalid operator `{:?}` for numbers", binary.operator),
                        )
                        .into()),
                    }
                } else if let (Value::Boolean(l), Value::Boolean(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Equals => Ok(Value::Boolean(l == r)),
                        ast::BinaryOperators::NotEquals => Ok(Value::Boolean(l != r)),
                        ast::BinaryOperators::And => Ok(Value::Boolean(*l && *r)),
                        ast::BinaryOperators::Or => Ok(Value::Boolean(*l || *r)),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("invalid operator `{:?}` for bools", binary.operator),
                        )
                        .into()),
                    }
                } else {
                    Err(RuntimeError::new(
                        TypeMismatch,
                        format!("incompatible operands {:?} and {:?}", left, right),
                    )
                    .into())
                }
            }
        }
    }
}
