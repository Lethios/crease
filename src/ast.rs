#[derive(Debug)]
pub struct Number(pub f64);

#[derive(Debug)]
pub enum UnaryOperators {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperators,
    pub operand: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOperators {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: BinaryOperators,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Number(Number),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}
