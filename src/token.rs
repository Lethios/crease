#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Identifier(String),
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    LParen,
    RParen,
    Print,
    Set,
    Semicolon,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize)) -> Self {
        Token { kind, span }
    }
}
