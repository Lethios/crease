use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Lex(LexerError),
    Parse(ParserError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lex(l) => write!(f, "Lexer error at column {}: {}", l.span.0, l),
            Error::Parse(p) => write!(f, "Parser error at column {}: {}", p.span.0, p),
        }
    }
}

impl From<LexerError> for Error {
    fn from(value: LexerError) -> Self {
        Error::Lex(value)
    }
}

impl From<ParserError> for Error {
    fn from(value: ParserError) -> Self {
        Error::Parse(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorKind {
    UnidentifiedCharacter,
}

#[derive(Debug, Clone, Copy)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub span: (usize, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LexerErrorKind::UnidentifiedCharacter => write!(f, "unidentified character"),
        }
    }
}

impl std::error::Error for LexerError {}

#[derive(Debug, Clone, Copy)]
pub enum ParserErrorKind {
    MissingDelimiter,
    UnidentifiedToken,
}

#[derive(Debug, Clone, Copy)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub span: (usize, usize),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ParserErrorKind::MissingDelimiter => write!(f, "missing delimiter"),
            ParserErrorKind::UnidentifiedToken => write!(f, "unidentified token"),
        }
    }
}

impl std::error::Error for ParserError {}
