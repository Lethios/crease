#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Identifier(String),
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Percent,
    Equals,
    LParen,
    RParen,
    Out,
    Set,
    Semicolon,
    Newline,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Token { kind, line, column }
    }
}
