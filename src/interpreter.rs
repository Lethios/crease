use crate::ast::{BinaryOperators, Expression, Statement, UnaryOperators};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    fn statement(&self, stmt: &Statement) {}

    fn expression(&self, expr: &Expression) -> f64 {
        match expr {
            Expression::Number(num) => num.0,

            Expression::UnaryOperation(unary) => match unary.operator {
                UnaryOperators::Add => self.expression(&unary.operand),
                UnaryOperators::Sub => -self.expression(&unary.operand),
            },

            Expression::BinaryOperation(binary) => match binary.operator {
                BinaryOperators::Add => {
                    self.expression(&binary.left) + self.expression(&binary.right)
                }
                BinaryOperators::Sub => {
                    self.expression(&binary.left) - self.expression(&binary.right)
                }
                BinaryOperators::Mul => {
                    self.expression(&binary.left) * self.expression(&binary.right)
                }
                BinaryOperators::Div => {
                    self.expression(&binary.left) / self.expression(&binary.right)
                }
            },
        }
    }
}
