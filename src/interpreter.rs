use std::collections::HashMap;

use crate::{
    ast::{BinaryOperators, Expression, Statement, UnaryOperators},
    error::{Error, RuntimeError, RuntimeErrorKind::UndefinedVariable},
};

#[derive(Default, Debug)]
pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Statement]) -> Result<(), Error> {
        for stmt in program {
            self.statement(stmt)?;
        }

        Ok(())
    }

    fn statement(&mut self, stmt: &Statement) -> Result<(), Error> {
        match stmt {
            Statement::Assignment(assign) => {
                let key = assign.iden.0.clone();
                let val = self.expression(&assign.expr)?;

                self.variables.insert(key, val);
            }
            Statement::Print(print) => {
                let val = self.expression(&print.0)?;
                println!("{}", val);
            }
        }

        Ok(())
    }

    fn expression(&self, expr: &Expression) -> Result<f64, Error> {
        match expr {
            Expression::Number(num) => Ok(num.0),

            Expression::Identifier(iden) => match self.variables.get(&iden.0) {
                Some(val) => Ok(*val),
                None => Err(RuntimeError::new(UndefinedVariable).into()),
            },

            Expression::UnaryOperation(unary) => {
                let value = self.expression(&unary.operand)?;

                match unary.operator {
                    UnaryOperators::Add => Ok(value),
                    UnaryOperators::Sub => Ok(-value),
                }
            }

            Expression::BinaryOperation(binary) => {
                let left = self.expression(&binary.left)?;
                let right = self.expression(&binary.right)?;

                match binary.operator {
                    BinaryOperators::Add => Ok(left + right),
                    BinaryOperators::Sub => Ok(left - right),
                    BinaryOperators::Mul => Ok(left * right),
                    BinaryOperators::Div => Ok(left / right),
                    BinaryOperators::IntDiv => Ok((left / right).trunc()),
                    BinaryOperators::Mod => Ok(left % right),
                }
            }
        }
    }
}
