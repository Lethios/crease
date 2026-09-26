use std::collections::HashMap;

use crate::{
    ast,
    error::{Error, RuntimeError, RuntimeErrorKind::*},
};

const ERROR: f64 = 1e-10;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(std::string::String),
    Array(Vec<Value>),
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
            ast::Statement::DeclareStmt(declare_stmt) => {
                let key = &declare_stmt.iden.value;
                let val = self.expression(&declare_stmt.expr)?;

                self.global_var.insert(key.to_string(), val);
            }
            ast::Statement::AssignmentStmt(assign_stmt) => {
                let key = &assign_stmt.iden.value;
                let val = self.expression(&assign_stmt.expr)?;

                if let Some(old_val) = self.global_var.get_mut(key) {
                    *old_val = val;
                } else {
                    return Err(RuntimeError::new(
                        UndefinedVariable,
                        format!("undeclared variable assignment `{}`", key),
                    )
                    .into());
                }
            }
            ast::Statement::IndexAssignmentStmt(idxassign_stmt) => {
                let key = &idxassign_stmt.iden.value;
                let idx = self.expression(&idxassign_stmt.idx)?;
                let val = self.expression(&idxassign_stmt.expr)?;

                if let Some(array) = self.global_var.get_mut(key) {
                    match array {
                        Value::Array(arr) => match idx {
                            Value::Int(i) => {
                                if i < 0 {
                                    return Err(RuntimeError::new(
                                        InvalidIndex,
                                        "array index cannot be negative".to_string(),
                                    )
                                    .into());
                                }

                                arr[i as usize] = val;
                            }
                            _ => {
                                return Err(RuntimeError::new(
                                    InvalidIndex,
                                    "array index must be an integer".to_string(),
                                )
                                .into());
                            }
                        },
                        _ => {
                            return Err(RuntimeError::new(
                                TypeMismatch,
                                format!("cannot index value of type `{:?}`", array),
                            )
                            .into());
                        }
                    }
                }
            }
            ast::Statement::DeleteStmt(delete_stmt) => {
                let key = &delete_stmt.iden.value;

                self.global_var.remove(key);
            }
            ast::Statement::InputStmt(input_stmt) => {
                let key = input_stmt.iden.value.clone();
                let mut value = String::new();
                std::io::stdin().read_line(&mut value).unwrap();

                self.global_var
                    .insert(key, Value::String(value.trim().to_string()));
            }
            ast::Statement::PrintStmt(print_stmt) => {
                let val = self.expression(&print_stmt.expr)?;

                match val {
                    Value::Int(num) => println!("{}", num),
                    Value::Float(num) => println!("{}", num),
                    Value::Boolean(boolean) => println!("{}", boolean),
                    Value::String(string) => println!("{}", string),
                    Value::Array(array) => println!("{:?}", array),
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
                        ("expected boolean in `if` condition").to_string(),
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
                            ("expected boolean in `while` condition").to_string(),
                        )
                        .into());
                    }
                }
            },
        }

        Ok(())
    }

    fn expression(&self, expr: &ast::Expression) -> Result<Value, Error> {
        match expr {
            ast::Expression::IntegerLiteral(n) => Ok(Value::Int(n.int)),

            ast::Expression::FloatLiteral(n) => Ok(Value::Float(n.float)),

            ast::Expression::BooleanLiteral(b) => Ok(Value::Boolean(b.boolean)),

            ast::Expression::StringLiteral(s) => Ok(Value::String(s.string.clone())),

            ast::Expression::ArrayLiteral(al) => Ok(Value::Array(al.arr)),

            ast::Expression::ArrayIndexing(ai) => Ok(ai.iden[ai.idx]),

            ast::Expression::Identifier(i) => match self.global_var.get(&i.value) {
                Some(val) => Ok(val.clone()),
                None => Err(RuntimeError::new(
                    UndefinedVariable,
                    format!("undefined variable `{}`", i.value),
                )
                .into()),
            },

            ast::Expression::UnaryOperation(unary) => {
                let value = self.expression(&unary.operand)?;

                match unary.operator {
                    ast::UnaryOperators::Add => match value {
                        Value::Int(_) | Value::Float(_) => Ok(value),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            "expected either `int` or `float` for unary `+`".to_string(),
                        )
                        .into()),
                    },
                    ast::UnaryOperators::Sub => match value {
                        Value::Int(num) => Ok(Value::Int(-num)),
                        Value::Float(num) => Ok(Value::Float(-num)),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            "expected either `int` or `float` for unary `-`".to_string(),
                        )
                        .into()),
                    },
                    ast::UnaryOperators::Not => {
                        if let Value::Boolean(boolean) = value {
                            Ok(Value::Boolean(!boolean))
                        } else {
                            Err(RuntimeError::new(
                                TypeMismatch,
                                format!("expected `boolean` for unary `!`, found {:?}", value),
                            )
                            .into())
                        }
                    }
                    ast::UnaryOperators::ToInt => match value {
                        Value::Int(_) => Ok(value),
                        Value::Float(num) => Ok(Value::Int(num as i64)),
                        Value::String(string) => {
                            let num = string.parse::<i64>().map_err(|_| {
                                RuntimeError::new(
                                    TypeMismatch,
                                    format!("failed to cast `{}` to int", string),
                                )
                            })?;
                            Ok(Value::Int(num))
                        }
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("cannot cast type `{:?}` to int", value),
                        )
                        .into()),
                    },
                    ast::UnaryOperators::ToFloat => match value {
                        Value::Int(num) => Ok(Value::Float(num as f64)),
                        Value::Float(_) => Ok(value),
                        Value::String(string) => {
                            let num = string.parse::<f64>().map_err(|_| {
                                RuntimeError::new(
                                    TypeMismatch,
                                    format!("failed to cast `{}` to float", string),
                                )
                            })?;
                            Ok(Value::Float(num))
                        }
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("cannot cast type `{:?}` to float", value),
                        )
                        .into()),
                    },
                    ast::UnaryOperators::ToBool => match value {
                        Value::Boolean(_) => Ok(value),
                        Value::String(string) => {
                            let boolean = string.parse::<bool>().map_err(|_| {
                                RuntimeError::new(
                                    TypeMismatch,
                                    format!("failed to cast `{}` to boolean", string),
                                )
                            })?;
                            Ok(Value::Boolean(boolean))
                        }
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("cannot cast type `{:?}` to boolean", value),
                        )
                        .into()),
                    },
                    ast::UnaryOperators::ToStr => match value {
                        Value::Int(num) => Ok(Value::String(num.to_string())),
                        Value::Float(num) => Ok(Value::String(num.to_string())),
                        Value::Boolean(boolean) => Ok(Value::String(boolean.to_string())),
                        Value::String(_) => Ok(value),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("cannot cast type `{:?}` to string", value),
                        )
                        .into()),
                    },
                }
            }

            ast::Expression::BinaryOperation(binary) => {
                let left = self.expression(&binary.left)?;

                // shortcircuit and/or
                match binary.operator {
                    ast::BinaryOperators::And => match left {
                        Value::Boolean(true) => {
                            let right = self.expression(&binary.right)?;
                            match right {
                                Value::Boolean(true) | Value::Boolean(false) => return Ok(right),
                                _ => {
                                    return Err(RuntimeError::new(
                                        TypeMismatch,
                                        format!(
                                            "incompatible operands `{:?}` and `{:?}`",
                                            left, right
                                        ),
                                    )
                                    .into());
                                }
                            }
                        }
                        Value::Boolean(false) => return Ok(left),
                        _ => {
                            return Err(RuntimeError::new(
                                TypeMismatch,
                                format!("invalid operator `{:?}` for booleans", binary.operator),
                            )
                            .into());
                        }
                    },
                    ast::BinaryOperators::Or => match left {
                        Value::Boolean(true) => return Ok(left),
                        Value::Boolean(false) => {
                            let right = self.expression(&binary.right)?;
                            match right {
                                Value::Boolean(true) | Value::Boolean(false) => return Ok(right),
                                _ => {
                                    return Err(RuntimeError::new(
                                        TypeMismatch,
                                        format!(
                                            "incompatible operands `{:?}` and `{:?}`",
                                            left, right
                                        ),
                                    )
                                    .into());
                                }
                            }
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                TypeMismatch,
                                format!("invalid operator `{:?}` for booleans", binary.operator),
                            )
                            .into());
                        }
                    },
                    _ => {}
                }

                let right = self.expression(&binary.right)?;

                if let (Value::Int(l), Value::Int(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Add => Ok(Value::Int(l + r)),
                        ast::BinaryOperators::Sub => Ok(Value::Int(l - r)),
                        ast::BinaryOperators::Mul => Ok(Value::Int(l * r)),
                        ast::BinaryOperators::Div => {
                            if r == &0 {
                                return Err(RuntimeError::new(
                                    DivisionByZero,
                                    "division by zero".to_string(),
                                )
                                .into());
                            }
                            Ok(Value::Int(l / r))
                        }
                        ast::BinaryOperators::Mod => {
                            if r == &0 {
                                return Err(RuntimeError::new(
                                    DivisionByZero,
                                    "division by zero".to_string(),
                                )
                                .into());
                            }
                            Ok(Value::Int(l % r))
                        }
                        ast::BinaryOperators::LThan => Ok(Value::Boolean(l < r)),
                        ast::BinaryOperators::GThan => Ok(Value::Boolean(l > r)),
                        ast::BinaryOperators::LThanEquals => Ok(Value::Boolean(l <= r)),
                        ast::BinaryOperators::GThanEquals => Ok(Value::Boolean(l >= r)),
                        ast::BinaryOperators::Equals => Ok(Value::Boolean(l == r)),
                        ast::BinaryOperators::NotEquals => Ok(Value::Boolean(l != r)),
                        _ => Err(RuntimeError::new(
                            TypeMismatch,
                            format!("invalid operator `{:?}` for ints", binary.operator),
                        )
                        .into()),
                    }
                } else if let (Value::Float(l), Value::Float(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Add => Ok(Value::Float(l + r)),
                        ast::BinaryOperators::Sub => Ok(Value::Float(l - r)),
                        ast::BinaryOperators::Mul => Ok(Value::Float(l * r)),
                        ast::BinaryOperators::Div => {
                            if r.abs() <= ERROR {
                                return Err(RuntimeError::new(
                                    DivisionByZero,
                                    "division by zero".to_string(),
                                )
                                .into());
                            }
                            Ok(Value::Float(l / r))
                        }
                        ast::BinaryOperators::Mod => {
                            if r.abs() <= ERROR {
                                return Err(RuntimeError::new(
                                    DivisionByZero,
                                    "division by zero".to_string(),
                                )
                                .into());
                            }
                            Ok(Value::Float(l % r))
                        }
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
                            format!("invalid operator `{:?}` for floats", binary.operator),
                        )
                        .into()),
                    }
                } else if let (Value::Boolean(l), Value::Boolean(r)) = (&left, &right) {
                    match binary.operator {
                        ast::BinaryOperators::Equals => Ok(Value::Boolean(l == r)),
                        ast::BinaryOperators::NotEquals => Ok(Value::Boolean(l != r)),
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
                        format!("incompatible operands `{:?}` and `{:?}`", left, right),
                    )
                    .into())
                }
            }
        }
    }
}
