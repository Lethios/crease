use crate::ast::{BinaryOperators, Expression, UnaryOperators};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expr: &Expression) -> f64 {
        match expr {
            Expression::Number(num) => num.0,

            Expression::UnaryOperation(unary) => match unary.operator {
                UnaryOperators::Add => self.interpret(&unary.operand),
                UnaryOperators::Sub => -self.interpret(&unary.operand),
            },

            Expression::BinaryOperation(binary) => match binary.operator {
                BinaryOperators::Add => {
                    self.interpret(&binary.left) + self.interpret(&binary.right)
                }
                BinaryOperators::Sub => {
                    self.interpret(&binary.left) - self.interpret(&binary.right)
                }
                BinaryOperators::Mul => {
                    self.interpret(&binary.left) * self.interpret(&binary.right)
                }
                BinaryOperators::Div => {
                    self.interpret(&binary.left) / self.interpret(&binary.right)
                }
            },
        }
    }
}
