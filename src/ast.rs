#[derive(Debug)]
pub struct Number(pub f64);

#[derive(Debug)]
pub struct Boolean(pub bool);

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum UnaryOperators {
    Add,
    Sub,
    Not,
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
    Mod,
    LThan,
    GThan,
    LThanEquals,
    GThanEquals,
    Equals,
    NotEquals,
    And,
    Or,
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
    Boolean(Boolean),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug)]
pub struct Assignment {
    pub iden: Identifier,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct Print(pub Expression);

#[derive(Debug)]
pub struct IfStmt {
    pub if_cond: Expression,
    pub if_stmt: Vec<Statement>,
    pub else_stmt: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    Print(Print),
    IfStmt(IfStmt),
}
