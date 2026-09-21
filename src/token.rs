#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    True,
    False,
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exclaim,

    Equals,

    LAngle,
    RAngle,
    LAngleEquals,
    RAngleEquals,

    DEquals,
    NotEquals,
    DAmpersand,
    DPipe,

    LParen,
    RParen,

    Out,
    Set,

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
