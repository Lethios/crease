#[derive(Debug, PartialEq)]
pub struct Number(pub f64);

#[derive(Debug, PartialEq)]
pub struct Boolean(pub bool);

#[derive(Debug, PartialEq)]
pub struct String(pub std::string::String);

#[derive(Debug, PartialEq)]
pub struct Identifier(pub std::string::String);

#[derive(Debug, PartialEq)]
pub enum UnaryOperators {
    Add,
    Sub,
    Not,
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperators,
    pub operand: Box<Expression>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub operator: BinaryOperators,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Boolean(Boolean),
    String(String),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub iden: Identifier,
    pub expr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Print(pub Expression);

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub if_cond: Expression,
    pub if_stmts: Vec<Statement>,
    pub else_stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStmt {
    pub while_cond: Expression,
    pub while_stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    Print(Print),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
}
