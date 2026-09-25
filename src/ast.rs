#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub int: i64,
}

#[derive(Debug, PartialEq)]
pub struct FloatLiteral {
    pub float: f64,
}

#[derive(Debug, PartialEq)]
pub struct BooleanLiteral {
    pub boolean: bool,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub string: std::string::String,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: std::string::String,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperators {
    Add,
    Sub,
    Not,
    ToInt,
    ToFloat,
    ToBool,
    ToStr,
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
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug, PartialEq)]
pub struct DeclareStmt {
    pub iden: Identifier,
    pub expr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStmt {
    pub iden: Identifier,
    pub expr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct DeleteStmt {
    pub iden: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct InputStmt {
    pub iden: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct PrintStmt {
    pub expr: Expression,
}

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
    DeclareStmt(DeclareStmt),
    AssignmentStmt(AssignmentStmt),
    DeleteStmt(DeleteStmt),
    InputStmt(InputStmt),
    PrintStmt(PrintStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
}
