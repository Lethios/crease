use std::collections::HashMap;

use crate::{
    ast,
    error::{Error, RuntimeError, RuntimeErrorKind::*},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(std::string::String),
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
            ast::Statement::Assignment(assign_stmt) => {
                let key = assign_stmt.iden.0.clone();
                let val = self.expression(&assign_stmt.expr)?;

                self.global_var.insert(key, val);
            }
            ast::Statement::Print(print_stmt) => {
                let val = self.expression(&print_stmt.0)?;

                match val {
                    Value::Number(num) => println!("{}", num),
                    Value::Boolean(boolean) => println!("{}", boolean),
                    Value::String(string) => println!("{}", string),
                }
            }
            ast::Statement::IfStmt(if_stmt) => match self.expression(&if_stmt.if_cond)? {
                Value::Boolean(boolean) => {
                    if boolean {
                        for stmt in &if_stmt.if_stmts {
                            self.statement(stmt)?;
                        }
                    } else {
                        for stmt in &if_stmt.else_stmts {
                            self.statement(stmt)?;
                        }
                    }
                }
                _ => {
                    return Err(RuntimeError::new(
                        TypeMismatch,
                        format!("expected boolean in `if` condition"),
                    )
                    .into());
                }
            },
            ast::Statement::WhileStmt(while_stmt) => loop {
                match self.expression(&while_stmt.while_cond)? {
                    Value::Boolean(true) => {
                        for stmt in &while_stmt.while_stmts {
                            self.statement(stmt)?;
                        }
                    }

                    Value::Boolean(false) => {
                        break;
                    }

                    _ => {
                        return Err(RuntimeError::new(
                            TypeMismatch,
                            format!("expected boolean in `while` condition"),
                        )
                        .into());
                    }
                }
            },
        }

        Ok(())
    }

    fn expression(&self, expr: &ast::Expression) -> Result<Value, Error> {
        const ERROR: f64 = 1e-10;

        match expr {
            ast::Expression::Number(num) => Ok(Value::Number(num.0)),

            ast::Expression::Boolean(boolean) => Ok(Value::Boolean(boolean.0)),

            ast::Expression::String(string) => Ok(Value::String(string.0.to_owned())),

            ast::Expression::Identifier(iden) => match self.global_var.get(&iden.0) {
                Some(val) => Ok(val.to_owned()),
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
                        if let Value::Boolean(boolean) = value {
                            Ok(Value::Boolean(!boolean))
                        } else {
                            Err(RuntimeError::new(
                                TypeMismatch,
                                format!("expected boolean for unary `!`, found {:?}", value),
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
                            format!("invalid operator `{:?}` for booleans", binary.operator),
                        )
                        .into()),
                    }
                } else if let (Value::String(l), Value::String(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Add => Ok(Value::String(format!("{l}{r}"))),
                        ast::BinaryOperators::Equals => Ok(Value::Boolean(l.eq(r))),
                        ast::BinaryOperators::NotEquals => Ok(Value::Boolean(l.ne(r))),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("invalid operator `{:?}` for strings", binary.operator),
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
