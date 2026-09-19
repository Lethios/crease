use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Lex(LexerError),
    Parse(ParserError),
    Runtime(RuntimeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lex(l) => write!(f, "Lexer error at column {}: {}", l.span.0, l),
            Error::Parse(p) => write!(f, "Parser error at column {}: {}", p.span.0, p),
            Error::Runtime(r) => write!(f, "Runtime error: {}", r),
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

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        Error::Runtime(value)
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

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: (usize, usize)) -> Self {
        Self { kind, span }
    }
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
    ExpectedStatement,
    UnexpectedToken,
    MissingSemicolon,
}

#[derive(Debug, Clone, Copy)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub span: (usize, usize),
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, span: (usize, usize)) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ParserErrorKind::MissingDelimiter => write!(f, "missing delimiter"),
            ParserErrorKind::UnidentifiedToken => write!(f, "unidentified token"),
            ParserErrorKind::ExpectedStatement => write!(f, "expected statement"),
            ParserErrorKind::UnexpectedToken => write!(f, "unexpected token"),
            ParserErrorKind::MissingSemicolon => write!(f, "unexpected token"),
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeErrorKind {
    UndefinedVariable,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            RuntimeErrorKind::UndefinedVariable => write!(f, "undefine variable"),
        }
    }
}
