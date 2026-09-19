use crate::{
    ast::{BinaryOperators, Expression, UnaryOperators},
    error::Error,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expr: &Expression) -> Result<f64, Error> {
        let res;

        match expr {
            Expression::Number(num) => res = num.0,
            Expression::UnaryOperation(unary) => match unary.operator {
                UnaryOperators::Add => res = self.interpret(&unary.operand)?,
                UnaryOperators::Sub => res = -self.interpret(&unary.operand)?,
            },
            Expression::BinaryOperation(binary) => match binary.operator {
                BinaryOperators::Add => {
                    res = self.interpret(&binary.left)? + self.interpret(&binary.right)?
                }
                BinaryOperators::Sub => {
                    res = self.interpret(&binary.left)? - self.interpret(&binary.right)?
                }
                BinaryOperators::Mul => {
                    res = self.interpret(&binary.left)? * self.interpret(&binary.right)?
                }
                BinaryOperators::Div => {
                    res = self.interpret(&binary.left)? / self.interpret(&binary.right)?
                }
            },
        }

        Ok(res)
    }
}
