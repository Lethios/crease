#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int,
    IntegerLiteral(i64),

    Float,
    FloatLiteral(f64),

    Bool,
    True,
    False,

    Str,
    StringLiteral(String),

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
    In,
    Set,
    Del,
    If,
    Else,
    EndIf,
    While,
    EndWhile,

    Colon,
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
