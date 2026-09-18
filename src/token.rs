#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize)) -> Self {
        Token { kind, span }
    }
}
