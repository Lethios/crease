#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Integer(i32),
    Plus,
    Minus,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}
