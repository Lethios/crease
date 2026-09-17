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
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}
